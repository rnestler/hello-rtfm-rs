#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use rtfm::{self, app, Instant};

use f3::{
    hal::prelude::*,
    led::Leds,
};

const PERIOD: u32 = 1_000_000;

#[app(device = stm32f30x)]
const APP: () = {
    static mut LEDS: Leds = ();

    #[init(schedule = [leds])]
    fn init() {
        // device and core get injected by RTFM
        let mut rcc = device.RCC.constrain();
        let gpioe = device.GPIOE.split(&mut rcc.ahb);
        let now = Instant::now();
        schedule.leds(now + PERIOD.cycles()).unwrap();
        LEDS = Leds::new(gpioe);
    }

    #[task(resources = [LEDS], schedule = [leds])]
    fn leds() {
        static mut curr: usize = 0;
        schedule.leds(scheduled + PERIOD.cycles()).unwrap();
        resources.LEDS[*curr].off();
        *curr += 1;
        if *curr > 7 {
            *curr = 0;
        }
        resources.LEDS[*curr].on();
    }

    extern "C" {
        fn SPI1();
    }
};

