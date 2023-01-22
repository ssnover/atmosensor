#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f1xx_hal::gpio::Edge;
use stm32f1xx_hal::gpio::ExtiPin;
use stm32f1xx_hal::i2c::Mode;
use stm32f1xx_hal::pac::{interrupt, Interrupt, NVIC};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{rcc::RccExt, usb::Peripheral};

mod cmd_handlers;
mod scd30;
mod static_resources;
mod tasks;
mod utils;

use static_resources::*;

#[entry]
fn main() -> ! {
    let mut device_peripherals = stm32f1xx_hal::pac::Peripherals::take().unwrap();
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

    let usb_handler = tasks::UsbHandler::new(usb);
    unsafe { CMD_QUEUE.write(tasks::CommandQueue::new()) };
    unsafe { USB_RESPONSE_QUEUE.write(tasks::CommandQueue::new()) };
    unsafe {
        ERROR_LED.write(gpiob.pb8.into_push_pull_output(&mut gpiob.crh));
    }

    unsafe { TEST_LED.write(gpiob.pb7.into_push_pull_output(&mut gpiob.crl)) };

    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = stm32f1xx_hal::i2c::I2c::i2c2(
        device_peripherals.I2C2,
        (scl, sda),
        Mode::Standard {
            frequency: 50.kHz(),
        },
        clocks,
    )
    .blocking(1000, 10, 1000, 1000, clocks);
    unsafe {
        I2C_BUS.write(i2c);
    };

    let mut scd_data_rdy_pin = gpiob.pb0.into_floating_input(&mut gpiob.crl);
    scd_data_rdy_pin.make_interrupt_source(&mut afio);
    scd_data_rdy_pin.trigger_on_edge(&mut device_peripherals.EXTI, Edge::Rising);
    scd_data_rdy_pin.enable_interrupt(&mut device_peripherals.EXTI);
    unsafe {
        SCD_DATA_RDY_PIN.write(scd_data_rdy_pin);
    }

    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }

    let command_handler = tasks::CommandHandler::new();

    loop {
        usb_handler.run();
        command_handler.run();
        delay(clocks.sysclk().raw() / 10);
    }
}

#[interrupt]
fn EXTI0() {
    let data_rdy_pin = unsafe { &mut *SCD_DATA_RDY_PIN.as_mut_ptr() };
    if data_rdy_pin.check_interrupt() {
        data_rdy_pin.clear_interrupt_pending_bit();
    }
}
