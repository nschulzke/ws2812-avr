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
use core::marker::PhantomData;

pub struct Nanos {}
pub struct Micros {}
pub struct Time<U, const V: u64> {
    _data: PhantomData<U>,
}

pub trait TimeVal {
    const V: u64;

    const NANOS: u64;
    const MICROS: u64;
}

impl<const V: u64> TimeVal for Time<Micros, V> {
    const V: u64 = V;

    const NANOS: u64 = V * 1000;
    const MICROS: u64 = V;
}

impl<const V: u64> TimeVal for Time<Nanos, V> {
    const V: u64 = V;

    const NANOS: u64 = V;
    const MICROS: u64 = V / 1000;
}
