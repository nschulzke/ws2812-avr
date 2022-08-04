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
use smart_leds::hsv::{hsv2rgb, Hsv};
use ws2812_avr::{GRB, WS2812};

const LED_COUNT: usize = 5;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut driver = WS2812::new(pins.d9.into_output());
    let mut buf: [GRB; LED_COUNT] = [GRB::default(); LED_COUNT];

    let mut cur_offset = 0;
    let mut hsv = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    loop {
        let rgb = hsv2rgb(hsv);
        buf[cur_offset] = GRB {
            g: rgb.g,
            r: rgb.r,
            b: rgb.b,
        };
        driver.write(&buf);
        cur_offset = (cur_offset + 1) % LED_COUNT;
        hsv.hue += 1;
        arduino_hal::delay_ms(25);
    }
}
