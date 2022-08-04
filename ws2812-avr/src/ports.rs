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
pub trait StaticPort {
    unsafe fn read() -> u8;
    unsafe fn write(value: u8);
}

pub trait StaticPin {
    const PIN_BIT_INDEX: u8;
    type Port: StaticPort;
}

macro_rules! static_pins {
    {$(
	($portt:ident, $portn:ident): {
	    $(($pin:ident, $index:literal)),*
	}
    ),*} => {
	$(
	    impl StaticPort for arduino_hal::pac::$portt {
		#[inline(always)]
		unsafe fn read() -> u8 {
		    (*Self::PTR).$portn.read().bits()
		}

		#[inline(always)]
		unsafe fn write(value: u8) {
		    (*Self::PTR).$portn.write(|f| f.bits(value));
		}
	    }

	    $(
		impl StaticPin for arduino_hal::hal::port::$pin {
		    const PIN_BIT_INDEX: u8 = $index;
		    type Port = arduino_hal::pac::$portt;
		}
	    )*
	)*

    };
}

#[rustfmt::skip]
#[cfg(any(feature = "atmega48p", feature = "atmega168", feature = "atmega328p"))]
static_pins! {
    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5),
	(PB6, 6),
	(PB7, 7)
    },

    (PORTC, portc): {
	(PC0, 0),
	(PC1, 1),
	(PC2, 2),
	(PC3, 3),
	(PC4, 4),
	(PC5, 5),
	(PC6, 6)
    },

    (PORTD, portd): {
	(PD0, 0),
	(PD1, 1),
	(PD2, 2),
	(PD3, 3),
	(PD4, 4),
	(PD5, 5),
	(PD6, 6),
	(PD7, 7)
    }
}

#[rustfmt::skip]
#[cfg(feature = "atmega328pb")]
static_pins! {
    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5),
	(PB6, 6),
	(PB7, 7)
    },

    (PORTC, portc): {
	(PC0, 0),
	(PC1, 1),
	(PC2, 2),
	(PC3, 3),
	(PC4, 4),
	(PC5, 5),
	(PC6, 6)
    },

    (PORTD, portd): {
	(PD0, 0),
	(PD1, 1),
	(PD2, 2),
	(PD3, 3),
	(PD4, 4),
	(PD5, 5),
	(PD6, 6),
	(PD7, 7)
    },

    (PORTE, porte): {
	(PE0, 0),
	(PE1, 1),
	(PE2, 2),
	(PE3, 3)
    }
}

#[rustfmt::skip]
#[cfg(feature = "atmega32u4")]
static_pins! {
    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5),
	(PB6, 6),
	(PB7, 7)
    },

    (PORTC, portc): {
	(PC6, 6),
	(PC7, 7)
    },

    (PORTD, portd): {
	(PD0, 0),
	(PD1, 1),
	(PD2, 2),
	(PD3, 3),
	(PD4, 4),
	(PD5, 5),
	(PD6, 6),
	(PD7, 7)
    },

    (PORTE, porte): {
	(PE2, 2),
	(PE6, 6)
    },

    (PORTF, portf): {
	(PF0, 0),
	(PF1, 1),
	(PF4, 4),
	(PF5, 5),
	(PF6, 6),
	(PF7, 7)
    }
}

#[rustfmt::skip]
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
static_pins! {
    (PORTA, porta): {
	(PA0, 0),
	(PA1, 1),
	(PA2, 2),
	(PA3, 3),
	(PA4, 4),
	(PA5, 5),
	(PA6, 6),
	(PA7, 7)
    },

    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5),
	(PB6, 6),
	(PB7, 7)
    },

    (PORTC, portc): {
	(PC0, 0),
	(PC1, 1),
	(PC2, 2),
	(PC3, 3),
	(PC4, 4),
	(PC5, 5),
	(PC6, 6),
	(PC7, 7)
    },

    (PORTD, portd): {
	(PD0, 0),
	(PD1, 1),
	(PD2, 2),
	(PD3, 3),
	(PD4, 4),
	(PD5, 5),
	(PD6, 6),
	(PD7, 7)
    },

    (PORTE, porte): {
	(PE0, 0),
	(PE1, 1),
	(PE2, 2),
	(PE3, 3),
	(PE4, 4),
	(PE5, 5),
	(PE6, 6),
	(PE7, 7)
    },

    (PORTF, portf): {
	(PF0, 0),
	(PF1, 1),
	(PF2, 2),
	(PF3, 3),
	(PF4, 4),
	(PF5, 5),
	(PF6, 6),
	(PF7, 7)
    },

    (PORTG, portg): {
	(PG0, 0),
	(PG1, 1),
	(PG2, 2),
	(PG3, 3),
	(PG4, 4),
	(PG5, 5)
    },

    (PORTH, porth): {
	(PH0, 0),
	(PH1, 1),
	(PH2, 2),
	(PH3, 3),
	(PH4, 4),
	(PH5, 5),
	(PH6, 6),
	(PH7, 7)
    },

    (PORTJ, portj): {
	(PJ0, 0),
	(PJ1, 1),
	(PJ2, 2),
	(PJ3, 3),
	(PJ4, 4),
	(PJ5, 5),
	(PJ6, 6),
	(PJ7, 7)
    },

    (PORTK, portk): {
	(PK0, 0),
	(PK1, 1),
	(PK2, 2),
	(PK3, 3),
	(PK4, 4),
	(PK5, 5),
	(PK6, 6),
	(PK7, 7)
    },

    (PORTL, portl): {
	(PL0, 0),
	(PL1, 1),
	(PL2, 2),
	(PL3, 3),
	(PL4, 4),
	(PL5, 5),
	(PL6, 6),
	(PL7, 7)
    }
}

#[rustfmt::skip]
#[cfg(feature = "attiny85")]
static_pins! {
    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5)
    }
}

#[rustfmt::skip]
#[cfg(feature = "attiny88")]
static_pins! {
    (PORTA, porta): {
	(PA0, 0),
	(PA1, 1),
	(PA2, 2),
	(PA3, 3)
    },

    (PORTB, portb): {
	(PB0, 0),
	(PB1, 1),
	(PB2, 2),
	(PB3, 3),
	(PB4, 4),
	(PB5, 5),
	(PB6, 6),
	(PB7, 7)
    },

    (PORTC, portc): {
	(PC0, 0),
	(PC1, 1),
	(PC2, 2),
	(PC3, 3),
	(PC4, 4),
	(PC5, 5),
	(PC6, 6),
	(PC7, 7)
    },

    (PORTD, portd): {
	(PD0, 0),
	(PD1, 1),
	(PD2, 2),
	(PD3, 3),
	(PD4, 4),
	(PD5, 5),
	(PD6, 6),
	(PD7, 7)
    }
}
