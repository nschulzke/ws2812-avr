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
macro_rules! lsl {
    ($reg:ident) => {
	core::arch::asm!("lsl {reg}", reg = inout(reg) $reg);
    };
}

macro_rules! dec {
    ($reg:ident) => {
	core::arch::asm!("dec {reg}", reg = inout(reg) $reg);
    };
}

macro_rules! branch_not_equal {
    ($tag:literal) => {
        core::arch::asm!(concat!("brne ", $tag));
    };
}

macro_rules! skip_if_bit_set {
    ($reg:ident, $bit:literal) => {
	core::arch::asm!(concat!("sbrs {reg}, ", $bit), reg = in(reg) $reg);
    };
}

macro_rules! ld_immediate {
    ($reg:ident, $value:literal) => {
	core::arch::asm!(concat!("ldi {reg}, ", $value), reg = out(reg_upper) $reg);
    };
}

macro_rules! asm_block {
    ($name:literal, $blk:block) => {
        core::arch::asm!(concat!($name, ":"));
        $blk;
    };
}

pub(crate) use asm_block;
pub(crate) use branch_not_equal;
pub(crate) use dec;
pub(crate) use ld_immediate;
pub(crate) use lsl;
pub(crate) use skip_if_bit_set;
