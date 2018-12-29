#![no_std]
#![no_main]

extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f30x;

use cortex_m_semihosting::hprintln;

use rtfm::{self, app, Instant};

use stm32f30x::{USART1};

use nb;

use f3::{
    hal::{prelude::*, serial::{self, Serial, Tx, Rx}},
    led::Leds,
};

const PERIOD: u32 = 1_000_000;

#[app(device = stm32f30x)]
const APP: () = {
    static mut CURRENT_LED: usize = 0;
    static mut LEDS: Leds = ();
    static mut SERIAL_TX: Tx<USART1> = ();
    static mut SERIAL_RX: Rx<USART1> = ();

    #[init(schedule = [leds])]
    fn init() {
        // device and core get injected by RTFM
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();

        let gpioe = device.GPIOE.split(&mut rcc.ahb);

        let now = Instant::now();
        schedule.leds(now + PERIOD.cycles()).unwrap();
        let mut gpioc = device.GPIOC.split(&mut rcc.ahb);

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
        let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

        let mut serial = Serial::usart1(device.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
        serial.listen(serial::Event::Rxne);
        let (tx, rx) = serial.split();

        LEDS = Leds::new(gpioe);
        SERIAL_TX = tx;
        SERIAL_RX = rx;
    }

    #[task(capacity = 4, resources = [SERIAL_TX, LEDS, CURRENT_LED])]
    fn uart_handler(byte: u8) {
        match resources.SERIAL_TX.write(byte) {
            Err(err) => {
                hprintln!("W: {:?}", err).unwrap();
            }
            _ => {}
        }

        let new_led = match byte {
            b'w' => 0,
            b'e' => 1,
            b'd' => 2,
            b'c' => 3,
            b'x' => 4,
            b'z' => 5,
            b'a' => 6,
            b'q' => 7,
            _ => *resources.CURRENT_LED,
        };
        if new_led != *resources.CURRENT_LED {
            resources.LEDS[*resources.CURRENT_LED].off();
            *resources.CURRENT_LED = new_led;
        }
    }

    #[task(resources = [LEDS, CURRENT_LED], schedule = [leds])]
    fn leds() {
        static mut on: bool = true;

        schedule.leds(scheduled + PERIOD.cycles()).unwrap();

        if *on {
            resources.LEDS[*resources.CURRENT_LED].off();
            *on = false;
        } else {
            resources.LEDS[*resources.CURRENT_LED].on();
            *on = true;
        }
    }

    #[interrupt(spawn = [uart_handler], resources = [SERIAL_RX])]
    fn USART1_EXTI25() {
        match resources.SERIAL_RX.read() {
            Ok(byte) =>  {
                spawn.uart_handler(byte).unwrap();
            }
            _ => {}
        }
    }

    extern "C" {
        fn SPI1();
    }
};

