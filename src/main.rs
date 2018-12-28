#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use stm32f30x::Interrupt;
use rtfm::app;

#[app(device = stm32f30x)]
const APP: () = {
    static mut SHARED: u32 = 0;

    #[init]
    fn init() {
        rtfm::pend(Interrupt::EXTI0);
    }

    // when omitted priority is assumed to be `1`
    #[interrupt(resources = [SHARED])]
    fn EXTI0() {
        hprintln!("A").unwrap();

        // the lower priority task requires a critical section to access the data
        resources.SHARED.lock(|shared| {
            // data can only be modified within this critical section (closure)
            *shared += 1;

            // GPIOB will *not* run right now due to the critical section
            rtfm::pend(Interrupt::SPI1);

            hprintln!("B - SHARED = {}", *shared).unwrap();

            // GPIOC does not contend for `SHARED` so it's allowed to run now
            rtfm::pend(Interrupt::SPI2);
        });

        // critical section is over: GPIOB can now start

        hprintln!("E").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[interrupt(priority = 2, resources = [SHARED])]
    fn SPI1() {
        // the higher priority task does *not* need a critical section
        *resources.SHARED += 1;

        hprintln!("D - SHARED = {}", *resources.SHARED).unwrap();
    }

    #[interrupt(priority = 3)]
    fn SPI2() {
        hprintln!("C").unwrap();
    }
};
