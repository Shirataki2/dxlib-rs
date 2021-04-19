use derive_more::Deref;
use num_traits::float::FloatCore;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deref)]
pub struct Angle<F>(F);

impl<F: FloatCore> Angle<F> {
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
