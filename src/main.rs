#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use cortex_m_semihosting::hprintln;

use rtfm::{self, app, Instant};

use stm32f30x::{USART1};

use nb;

use f3::{
    hal::{prelude::*, serial::{Serial, Tx, Rx}},
    led::Leds,
};

const PERIOD: u32 = 1_000_000;
const USART_PERIOD: u32 = 1_000;

#[app(device = stm32f30x)]
const APP: () = {
    static mut LEDS: Leds = ();
    static mut SERIAL_TX: Tx<USART1> = ();
    static mut SERIAL_RX: Rx<USART1> = ();

    #[init(schedule = [leds, uart_echo])]
    fn init() {
        // device and core get injected by RTFM
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();

        let gpioe = device.GPIOE.split(&mut rcc.ahb);

        let now = Instant::now();
        schedule.uart_echo(now).unwrap();
        schedule.leds(now + PERIOD.cycles()).unwrap();
        let mut gpioc = device.GPIOC.split(&mut rcc.ahb);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
        let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

        let serial = Serial::usart1(device.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
        let (tx, rx) = serial.split();

        LEDS = Leds::new(gpioe);
        SERIAL_TX = tx;
        SERIAL_RX = rx;
    }

    #[task(resources = [SERIAL_TX, SERIAL_RX], schedule = [uart_echo])]
    fn uart_echo() {
        match resources.SERIAL_RX.read() {
            Ok(byte) => {
                match resources.SERIAL_TX.write(byte) {
                    Err(err) => {
                        hprintln!("W: {:?}", err).unwrap();
                    }
                    _ => {}
                }
            }
            Err(nb::Error::WouldBlock) => {}
            Err(err) => {
                hprintln!("R: {:?}", err).unwrap();
            }
        }
        schedule.uart_echo(scheduled + USART_PERIOD.cycles()).unwrap();
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

