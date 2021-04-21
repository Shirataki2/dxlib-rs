use derive_more::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num::cast;
use num_traits::{float::FloatCore, FloatConst};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Deref,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
)]
pub struct Angle<F>(F);

impl<F: FloatCore + FloatConst> Angle<F> {
    pub fn normalized(&self) -> F {
        self.0 % (cast::<f32, F>(2.0f32).unwrap() * F::PI())
    }

    pub fn from_degrees(deg: F) -> Angle<F> {
        Self(Self::deg2rad(deg))
    }

    pub fn from_radians(rad: F) -> Angle<F> {
        Self(rad)
    }

    pub fn rad2deg(rad: F) -> F {
        FloatCore::to_degrees(rad)
    }

    pub fn deg2rad(deg: F) -> F {
        FloatCore::to_radians(deg)
    }
}
