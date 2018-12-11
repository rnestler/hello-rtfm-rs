#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_semihosting::hprintln;
use rtfm::app;

#[app(device = stm32f30x)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();
        loop {}
    }
};

