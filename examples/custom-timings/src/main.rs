/*
This file is part of ws2812-avr.

ws2812-avr is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

ws2812-avr is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with ws2812-avr. If not, see <https://www.gnu.org/licenses/>.
 */
#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use panic_halt as _;
use ws2812_avr::{util::time, Timings, GRB, WS2812};

const LED_COUNT: usize = 5;

/**
 * Timings that makes faster the update of each led, by reducing the
 * duration of each cycle and the length of each pulse, moving all the
 * timing parameters to the limits specified in this datasheet:
 * https://cdn-shop.adafruit.com/datasheets/WS2812.pdf. It works for
 * me on a small strip of 5 leds.
 */
pub struct UltraFastTimings {}

impl Timings for UltraFastTimings {
    type Rst = time::Time<time::Micros, 50>;
    type Cycle = time::Time<time::Nanos, 650>;
    type T1h = time::Time<time::Nanos, 550>;
    type T0h = time::Time<time::Nanos, 100>;
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut driver = WS2812::new_with_timings::<UltraFastTimings, _, _>(pins.d9.into_output());
    let buf_on: [GRB; LED_COUNT] = [GRB {
        g: 255,
        r: 255,
        b: 255,
    }; LED_COUNT];
    let buf_off: [GRB; LED_COUNT] = [GRB { g: 0, r: 0, b: 0 }; LED_COUNT];
    loop {
        driver.write(&buf_on);
        arduino_hal::delay_ms(1000);
        driver.write(&buf_off);
        arduino_hal::delay_ms(1000);
    }
}
