#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::gpiob;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::pac::{interrupt, Interrupt, NVIC};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    rcc::RccExt,
    usb::{Peripheral, UsbBus, UsbBusType},
};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;
static mut RED_LED: Option<gpiob::PB8<Output<PushPull>>> = None;

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
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

    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);
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

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    let mut buf = [0u8; 8];

    match serial.read(&mut buf) {
        Ok(count) => {
            if count > 0 {
                for ch in buf[0..count].iter_mut() {
                    if 0x61 <= *ch && *ch <= 0x7a {
                        *ch &= !0x20;
                    }
                    if *ch == 0x52 {
                        unsafe { RED_LED.as_mut().unwrap().toggle() };
                    }
                }
                serial.write(&buf[0..count]).ok();
            }
        }
        _ => {}
    }
}
