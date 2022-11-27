#![no_std]
#![no_main]

use core::borrow::Borrow;
use core::mem::MaybeUninit;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f1xx_hal::gpio::Edge;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::gpio::Floating;
use stm32f1xx_hal::gpio::Input;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::i2c::Mode;
use stm32f1xx_hal::pac::{interrupt, Interrupt, NVIC};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    rcc::RccExt,
    usb::{Peripheral, UsbBus, UsbBusType},
};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

mod scd30;
mod utils;
use utils::SpscQueue;

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;
static mut RED_LED: Option<gpiob::PB8<Output<PushPull>>> = None;
static mut SCD_DATA_RDY_PIN: MaybeUninit<gpiob::PB0<Input<Floating>>> = MaybeUninit::uninit();
static USB_RX_BUFFER: Mutex<RefCell<[u8; 1024]>> = Mutex::new(RefCell::new([0u8; 1024]));
static USB_TX_BUFFER: Mutex<RefCell<[u8; 1024]>> = Mutex::new(RefCell::new([0u8; 1024]));
static USB_ENCODING_BUFFER: Mutex<RefCell<[u8; 1024]>> = Mutex::new(RefCell::new([0u8; 1024]));

#[entry]
fn main() -> ! {
    let device_peripherals = stm32f1xx_hal::pac::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let rcc = device_peripherals.RCC.constrain();
    // Set with the Clock Configuration tab of STM32CubeMX
    let clocks = rcc
        .cfgr
        .use_hse(16.MHz())
        .sysclk(48.MHz())
        .hclk(48.MHz())
        .pclk1(24.MHz())
        .pclk2(48.MHz())
        .freeze(&mut flash.acr);
    assert!(clocks.usbclk_valid());

    let mut gpioa = device_peripherals.GPIOA.split();
    let mut gpiob = device_peripherals.GPIOB.split();
    let mut afio = device_peripherals.AFIO.constrain();

    // Pulling the D+ pin low to indicate a RESET condition on USB bus.
    // This triggers host to cut power which will allow device to re-enumerate
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();
    delay(clocks.sysclk().raw() / 100);

    let usb = Peripheral {
        usb: device_peripherals.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };

    unsafe {
        let usb_bus = UsbBus::new(usb);
        USB_BUS = Some(usb_bus);
        USB_SERIAL = Some(SerialPort::new(USB_BUS.as_ref().unwrap()));
        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Snostorm Labs")
            .product("Atmosensor")
            .serial_number("FAKE")
            .device_class(USB_CLASS_CDC)
            .build();
        USB_DEVICE = Some(usb_dev);
        RED_LED = Some(gpiob.pb8.into_push_pull_output(&mut gpiob.crh));
    }

    let mut green_led = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);

    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);

    let mut i2c = stm32f1xx_hal::i2c::I2c::i2c2(
        device_peripherals.I2C2,
        (scl, sda),
        Mode::Standard {
            frequency: 50.kHz(),
        },
        clocks,
    )
    .blocking(1000, 10, 1000, 1000, clocks);

    let scd_sensor = scd30::Scd30::new();
    let _ = scd_sensor.soft_reset(&mut i2c);

    {
        let scd_data_rdy_pin = unsafe { &mut *SCD_DATA_RDY_PIN.as_mut_ptr() };
        *scd_data_rdy_pin = gpiob.pb0.into_floating_input(&mut gpiob.crl);
        scd_data_rdy_pin.make_interrupt_source(&mut afio);
        scd_data_rdy_pin.trigger_on_edge(&device_peripherals.EXTI, Edge::Rising);
        scd_data_rdy_pin.enable_interrupt(&device_peripherals.EXTI);
    }

    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);
        NVIC::unmask(Interrupt::EXTI0);
    }

    loop {
        delay(clocks.sysclk().raw() / 10);
        green_led.set_high();
        delay(clocks.sysclk().raw() / 10);
        green_led.set_low();
    }
}

#[interrupt]
fn USB_HP_CAN_TX() {
    usb_interrupt();
}

#[interrupt]
fn USB_LP_CAN_RX0() {
    usb_interrupt();
}

#[interrupt]
fn EXTI0() {
    let data_rdy_pin = unsafe { &mut *SCD_DATA_RDY_PIN.as_mut_ptr() };
    if data_rdy_pin.check_interrupt() {
        unsafe { RED_LED.as_mut().unwrap().toggle() };
        data_rdy_pin.clear_interrupt_pending_bit();
    }
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    cortex_m::interrupt::free(|cs| {
        let mut rx_buffer_borrow = USB_RX_BUFFER.borrow(cs).borrow_mut();
        let mut rx_buffer = rx_buffer_borrow.as_mut();
        let mut tx_buffer_borrow = USB_TX_BUFFER.borrow(cs).borrow_mut();
        let mut tx_buffer = tx_buffer_borrow.as_mut();
        let mut encoding_buffer_borrow = USB_ENCODING_BUFFER.borrow(cs).borrow_mut();
        let mut encoding_buffer = encoding_buffer_borrow.as_mut();
        if let Ok(bytes_read) = serial.read(&mut rx_buffer) {
            if let Ok(bytes_decoded) = cobs::decode(&rx_buffer[..bytes_read], &mut encoding_buffer) {
                if bytes_decoded == 2 && encoding_buffer[0] == 0xde && encoding_buffer[1] == 0x00 {
                    // Received a ping, send a pong
                    encoding_buffer[0] = 0xde;
                    encoding_buffer[1] = 0x01;

                    let bytes_encoded = cobs::encode(&encoding_buffer[..2], &mut tx_buffer);
                    // TODO check for errors here
                    let _ = serial.write(&tx_buffer[..bytes_encoded]);
                }
            }
        }
    });
}
