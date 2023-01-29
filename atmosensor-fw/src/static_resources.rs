use core::mem::MaybeUninit;
use embedded_hal::blocking::i2c::{Read, Write};
use stm32f1xx_hal::gpio::Alternate;
use stm32f1xx_hal::gpio::Floating;
use stm32f1xx_hal::gpio::Input;
use stm32f1xx_hal::gpio::OpenDrain;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::i2c::BlockingI2c;
use stm32f1xx_hal::pac::I2C2;

use crate::tasks::CommandQueue;

// HARDWARE RESOURCES

pub static mut ERROR_LED: MaybeUninit<Pin<'B', 8, Output<PushPull>>> = MaybeUninit::uninit();
pub static mut TEST_LED: MaybeUninit<Pin<'B', 7, Output<PushPull>>> = MaybeUninit::uninit();
pub static mut I2C_BUS: MaybeUninit<
    BlockingI2c<
        I2C2,
        (
            Pin<'B', 10, Alternate<OpenDrain>>,
            Pin<'B', 11, Alternate<OpenDrain>>,
        ),
    >,
> = MaybeUninit::uninit();
pub static mut SCD_DATA_RDY_PIN: MaybeUninit<Pin<'B', 0, Input<Floating>>> = MaybeUninit::uninit();

pub trait I2cWriteAndRead<E>: Read<Error = E> + Write<Error = E> {}
impl<E, T: Read<Error = E> + Write<Error = E>> I2cWriteAndRead<E> for T {}

#[inline]
pub unsafe fn with_i2c_bus<F, R>(f: F) -> R
where
    F: FnOnce(&mut dyn I2cWriteAndRead<stm32f1xx_hal::i2c::Error>) -> R,
{
    f(I2C_BUS.assume_init_mut())
}

// SOFTWARE RESOURCES

pub static mut CMD_QUEUE: MaybeUninit<CommandQueue<48>> = MaybeUninit::uninit();
pub static mut USB_RESPONSE_QUEUE: MaybeUninit<CommandQueue<12>> = MaybeUninit::uninit();
