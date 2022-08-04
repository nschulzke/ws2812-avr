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
use crate::util::Sealed;

pub trait ColorOrder: Sealed {}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct GRB {
    pub g: u8,
    pub r: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BGR {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Sealed for RGB {}
impl ColorOrder for RGB {}

impl Sealed for GRB {}
impl ColorOrder for GRB {}

impl Sealed for BGR {}
impl ColorOrder for BGR {}
