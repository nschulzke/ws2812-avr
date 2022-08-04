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
use core::{arch::asm, marker::PhantomData};

// This file allows to generate block of NOP instructions based on the
// value of a const generic type. It is done by using a simple
// implementation of Peano numbers, for being able to count how many
// NOPs should be generated using type recursion. Then, the
// "IntoNopPeano" trait is provided along with the "Num" struct that
// allows to convert between common numbers and Peano ones.

// This could have been done using const expressions and const
// generics directly, but due to an opened in the Rust compiler it was
// giving me a lot of headaches to actually make this to be able to
// link successfully, so this is just a workaround for that issue:
// https://github.com/rust-lang/rust/issues/84669.

pub trait IsTrue {}
pub struct BExpr<const B: bool> {}
impl IsTrue for BExpr<true> {}

pub trait NopGen {
    fn gen();
}

pub struct Num<const V: u8> {}
pub struct Zero {}
pub struct Succ<N> {
    _n: PhantomData<N>,
}

// This trait is specialized on returning Peano numbers that are
// verified that return instances that can generate NOP blocks,
// instead of using a generic definition, because some issues I've
// encountered while trying to apply this type restriction in other
// places in the code.
pub trait IntoNopPeano {
    type Peano: NopGen;
}

impl IntoNopPeano for Num<0> {
    type Peano = Zero;
}

impl<const X: u8> IntoNopPeano for Num<X>
where
    BExpr<{ X > 0 }>: IsTrue,
    Num<{ X - 1 }>: IntoNopPeano,
{
    type Peano = Succ<<Num<{ X - 1 }> as IntoNopPeano>::Peano>;
}

impl NopGen for Zero {
    fn gen() {}
}

impl<X> NopGen for Succ<X>
where
    X: NopGen,
{
    #[inline(always)]
    default fn gen() {
        unsafe {
            asm!("nop");
        }

        <X as NopGen>::gen();
    }
}

impl<X> NopGen for Succ<Succ<X>>
where
    X: NopGen,
{
    #[inline(always)]
    fn gen() {
        unsafe {
            // This is an AVR instruction that is equivalent to doing nothing
            // (relative jump to +0, which basically means doing nothing useful)
            // but takes 2 cycles to complete. We can use it to save a
            // few bytes by collapsing two NOP cycles into a single
            // instruction.
            asm!("rjmp +0");
        }

        <X as NopGen>::gen();
    }
}
