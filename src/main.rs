#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use rtfm::app;

#[app(device = stm32f30x)]
const APP: () = {
    #[init]
    fn init() {}
};

