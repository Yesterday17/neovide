use std::{
    fmt::Display,
    ops::{Div, Mul},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use winit::dpi::PhysicalSize;

use crate::settings;

// Maybe this should be independent from serialization?
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u64,
    pub height: u64,
}

impl Default for Dimensions {
    fn default() -> Self {
        settings::DEFAULT_WINDOW_GEOMETRY
    }
}

impl FromStr for Dimensions {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        settings::parse_window_geometry(s)
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

macro_rules! impl_from_tuple_to_dimensions {
    ($type:ty) => {
        impl From<($type, $type)> for Dimensions {
            fn from((width, height): ($type, $type)) -> Self {
                Dimensions {
                    width: width as u64,
                    height: height as u64,
                }
            }
        }
    };
}

impl_from_tuple_to_dimensions!(u64);
impl_from_tuple_to_dimensions!(f32);

macro_rules! impl_from_dimensions_to_tuple {
    ($type:ty) => {
        impl From<Dimensions> for ($type, $type) {
            fn from(dimensions: Dimensions) -> Self {
                (dimensions.width as $type, dimensions.height as $type)
            }
        }
    };
}

impl_from_dimensions_to_tuple!(u64);
impl_from_dimensions_to_tuple!(u32);
impl_from_dimensions_to_tuple!(i32);

impl From<PhysicalSize<u32>> for Dimensions {
    fn from(PhysicalSize { width, height }: PhysicalSize<u32>) -> Self {
        Dimensions {
            width: width as u64,
            height: height as u64,
        }
    }
}

impl From<Dimensions> for PhysicalSize<u32> {
    fn from(Dimensions { width, height }: Dimensions) -> Self {
        PhysicalSize {
            width: width as u32,
            height: height as u32,
        }
    }
}

impl Mul for Dimensions {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Dimensions::from((self.width * other.width, self.height * other.height))
    }
}

impl Div for Dimensions {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Dimensions::from((self.width / other.width, self.height / other.height))
    }
}

impl Mul<Dimensions> for (u64, u64) {
    type Output = Self;

    fn mul(self, other: Dimensions) -> Self {
        let (x, y) = self;
        (x * other.width, y * other.height)
    }
}
