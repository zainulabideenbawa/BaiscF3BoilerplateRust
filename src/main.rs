#![no_main]
#![no_std]
extern crate panic_halt;
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::hal::prelude::*;
pub use f3::hal::stm32f30x::{self, GPIOA, RCC};
pub use f3::hal::{delay::Delay, prelude, time::MonoTimer};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f30x::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);
    let MonoTim = MonoTimer::new(cp.DWT, clocks);
    let stim = &mut cp.ITM.stim[0];

    iprint!(stim, "Hello");

    loop {}
}
