use std::fmt;

use derive_more::{Deref, DerefMut};

mod orbit;
pub use orbit::*;

pub type Vector = nalgebra::Vector2<f64>;

macro_rules! unit {
    ($name:ident, $unit:literal) => {
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
                unimplemented!()
            }
        }
    };
}

unit!(Length, "m");
unit!(Mass, "g");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(pub i64);
