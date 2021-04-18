use dxlib_sys::dx_GetColor;

use crate::math::vector::{Vector3, Vector4};

pub trait ColorElement: Sized + Clone + Copy {
    fn min() -> Self;
    fn max() -> Self;
}

impl ColorElement for u8 {
    fn min() -> Self {
        0
    }

    fn max() -> Self {
        255
    }
}

impl ColorElement for f32 {
    fn min() -> Self {
        0.0
    }

    fn max() -> Self {
        1.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color<T: ColorElement> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: ColorElement> Color<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl Color<u8> {
    pub fn to_u32(&self) -> u32 {
        unsafe { dx_GetColor(self.r as i32, self.g as i32, self.b as i32) }
    }
}

impl<T: ColorElement> Default for Color<T> {
    fn default() -> Self {
        Self::new(T::min(), T::min(), T::min(), T::max())
    }
}

impl<T: ColorElement> From<&[T; 4]> for Color<T> {
    fn from(v: &[T; 4]) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3],
        }
    }
}

impl<T: ColorElement> From<&[T; 3]> for Color<T> {
    fn from(v: &[T; 3]) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
            a: T::max(),
        }
    }
}

impl<T: ColorElement> From<Vector4<T>> for Color<T> {
    fn from(v: Vector4<T>) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3],
        }
    }
}

impl<T: ColorElement> From<Vector3<T>> for Color<T> {
    fn from(v: Vector3<T>) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
            a: T::max(),
        }
    }
}

impl From<Color<u8>> for Color<f32> {
    fn from(c: Color<u8>) -> Color<f32> {
        Color {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: c.a as f32 / 255.0,
        }
    }
}

impl From<Color<f32>> for Color<u8> {
    fn from(c: Color<f32>) -> Color<u8> {
        Color {
            r: (c.r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (c.g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (c.b.clamp(0.0, 1.0) * 255.0) as u8,
            a: (c.a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

#[test]
fn color_create() {
    let v = [0, 255, 0];
    let c = Color::from(&v);
    println!("{:?}", c);
}

impl Color<u8> {
    pub fn black() -> Color<u8> {
        Self::new(0, 0, 0, 255)
    }

    pub fn white() -> Color<u8> {
        Self::new(255, 255, 255, 255)
    }

    pub fn red() -> Color<u8> {
        Self::new(255, 0, 0, 255)
    }

    pub fn green() -> Color<u8> {
        Self::new(0, 255, 0, 255)
    }

    pub fn blue() -> Color<u8> {
        Self::new(0, 0, 255, 255)
    }

    pub fn magenta() -> Color<u8> {
        Self::new(255, 0, 255, 255)
    }

    pub fn cyan() -> Color<u8> {
        Self::new(0, 255, 255, 255)
    }

    pub fn yellow() -> Color<u8> {
        Self::new(255, 255, 0, 255)
    }
}
