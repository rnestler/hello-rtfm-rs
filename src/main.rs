#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use rtfm::{self, app, Instant};

use stm32f30x::{USART1};

use f3::{
    hal::{prelude::*, serial::Serial, gpio::{self, gpioa}},
    led::Leds,
};

const PERIOD: u32 = 1_000_000;

#[app(device = stm32f30x)]
const APP: () = {
    static mut LEDS: Leds = ();
    static mut SERIAL: Serial<USART1,(gpioa::PA9<gpio::AF7>, gpioa::PA10<gpio::AF7>) > = ();

    #[init(schedule = [leds])]
    fn init() {
        // device and core get injected by RTFM
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();

        let gpioe = device.GPIOE.split(&mut rcc.ahb);

        let now = Instant::now();
        schedule.leds(now + PERIOD.cycles()).unwrap();
        let mut gpioa = device.GPIOA.split(&mut rcc.ahb);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
        let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

        let serial = Serial::usart1(device.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

        LEDS = Leds::new(gpioe);
        SERIAL = serial;
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

