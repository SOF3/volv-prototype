use std::fmt;

use derive_more::{Deref, DerefMut};

mod orbit;
pub use orbit::*;

pub type Vector = nalgebra::Vector2<f64>;

macro_rules! unit {
    ($name:ident, $unit:literal, $base:literal) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            PartialOrd,
            Deref,
            DerefMut,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub struct $name(pub f64);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let (mag, prefix) = to_sci(self.0 * $base);
                write!(f, concat!("{}{}", $unit), mag, prefix)
            }
        }
    };
}

fn to_sci(mut value: f64) -> (f64, &'static str) {
    let mut exp = 0;
    while value >= 10000.0 && exp < 5 {
        value /= 1000.0;
        exp += 1;
    }
    let prefix = match exp {
        0 => "",
        1 => "k",
        2 => "M",
        3 => "G",
        4 => "T",
        5 => "P",
        _ => unreachable!(),
    };
    value = (value * 10.0).round() / 10.0;
    (value, prefix)
}

unit!(Length, "m", 1.0);
unit!(Mass, "g", 1000.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(pub i32);
