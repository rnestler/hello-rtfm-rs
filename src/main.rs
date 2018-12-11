#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use cortex_m_semihosting::hprintln;
use stm32f30x::interrupt;
use rtfm::{self, app};

use f3::{
    hal::{delay::Delay, prelude::*},
    led::Leds,
};

#[app(device = stm32f30x)]
const APP: () = {
    static mut LEDS: Leds = ();
    static mut DELAY: Delay = ();

    #[init]
    fn init() {
        // device and core get injected by RTFM
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let gpioe = device.GPIOE.split(&mut rcc.ahb);
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        LEDS = Leds::new(gpioe);
        DELAY = Delay::new(core.SYST, clocks);
    }

    #[idle(resources = [LEDS, DELAY])]
    fn idle() -> ! {
        let n = resources.LEDS.len();
        loop {
            for curr in 0..n {
                let next = (curr + 1) % n;
                resources.LEDS[curr].off();
                resources.LEDS[next].on();

                resources.DELAY.delay_ms(100_u8);
            }
        }
    }

    #[interrupt]
    fn SPI1() {
        static mut TIMES: u32 = 0;

        // Safe access to local `static mut` variable
        *TIMES += 1;

        hprintln!(
            "SPI1 called {} time{}",
            *TIMES,
            if *TIMES > 1 { "s" } else { "" }
        ).unwrap();
    }
};

