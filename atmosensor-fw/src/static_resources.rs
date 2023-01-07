use core::mem::MaybeUninit;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::PushPull;

pub static mut TEST_LED: MaybeUninit<Pin<'B', 7, Output<PushPull>>> = MaybeUninit::uninit();
