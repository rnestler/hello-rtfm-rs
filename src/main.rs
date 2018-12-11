#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use cortex_m_semihosting::hprintln;
use stm32f30x::{interrupt, Interrupt};
use rtfm::app;

#[app(device = stm32f30x)]
const APP: () = {
    static mut SHARED: u32 = 0; // A resource

    #[init]
    fn init() {
        rtfm::pend(Interrupt::SPI1);
        rtfm::pend(Interrupt::SPI2);
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();
        // *resources.SHARED += 1; // doesn't compile
        loop {}
    }

    #[interrupt(resources = [SHARED])]
    fn SPI1() {
        *resources.SHARED += 1;
        hprintln!("SPI1: SHARED = {}", resources.SHARED).unwrap();
    }

    #[interrupt(resources = [SHARED])]
    fn SPI2() {
        *resources.SHARED += 1;
        hprintln!("SPI2: SHARED = {}", resources.SHARED).unwrap();
    }
};

