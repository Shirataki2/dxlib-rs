use super::DotProduct;
use num_traits::{Float, One, Zero};
use std::ops::*;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T, const DIM: usize>(pub(crate) [T; DIM]);

impl<T, const DIM: usize> DotProduct<Vector<T, DIM>> for Vector<T, DIM>
where
    T: Zero + Mul<Output = T> + AddAssign + Copy,
{
    type Output = T;
    fn dot(self, rhs: Vector<T, DIM>) -> T {
        let mut ret = T::zero();
        for i in 0..DIM {
            ret += self[i] * rhs[i];
        }
        ret
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Zero + Mul<Output = T> + AddAssign + Copy,
{
    pub fn sq_magnitude(self) -> T {
        self.dot(self)
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Zero + Mul<Output = T> + AddAssign + Copy + Float,
{
    pub fn magnitude(self) -> T {
        self.sq_magnitude().sqrt()
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Zero + Mul<Output = T> + AddAssign + Copy + Float + One + Div<T, Output = T>,
    Self: Mul<T, Output = Self>,
{
    pub fn normalized(self) -> Self {
        self * (T::one() / self.magnitude())
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    Self: Copy + Sub<Output = Self> + Mul<T, Output = Self> + Add<Self, Output = Self>,
{
    pub fn lerp(self, rhs: Vector<T, DIM>, t: T) -> Vector<T, DIM> {
        self + (rhs - self) * t
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Copy + Zero + Mul<Output = T> + Sub<Output = T> + AddAssign,
{
    pub fn sq_distance(self, rhs: Vector<T, DIM>) -> T {
        let mut ret = T::zero();
        for i in 0..DIM {
            ret += (rhs[i] - self[i]) * (rhs[i] - self[i]);
        }
        ret
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Copy + Zero + Mul<Output = T> + Sub<Output = T> + AddAssign + Float,
{
    pub fn distance(self, rhs: Vector<T, DIM>) -> T {
        self.sq_distance(rhs).sqrt()
    }
}

impl<T, const DIM: usize> Zero for Vector<T, DIM>
where
    T: Copy + Zero + AddAssign + Add<Output = T>,
{
    fn is_zero(&self) -> bool {
        let mut v = true;
        for i in 0..DIM {
            v &= self[i].is_zero();
        }
        v
    }
    fn zero() -> Vector<T, DIM> {
        Vector::from([T::zero(); DIM])
    }
}

impl<T, const DIM: usize> One for Vector<T, DIM>
where
    T: Copy + One + MulAssign + Mul<T, Output = T> + PartialEq,
{
    fn is_one(&self) -> bool {
        let mut v = true;
        for i in 0..DIM {
            v &= self[i].is_one();
        }
        v
    }
    fn one() -> Vector<T, DIM> {
        Vector::from([T::one(); DIM])
    }
}

impl<T, const DIM: usize> From<[T; DIM]> for Vector<T, DIM>
where
    T: Clone + Copy,
{
    fn from(v: [T; DIM]) -> Self {
        Self(v)
    }
}

impl<T, const DIM: usize> Default for Vector<T, DIM>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self([T::default(); DIM])
    }
}

impl<T, const DIM: usize> Index<usize> for Vector<T, DIM> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T, const DIM: usize> IndexMut<usize> for Vector<T, DIM> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}

impl<T: AddAssign + Clone, const DIM: usize> AddAssign<Vector<T, DIM>> for Vector<T, DIM> {
    fn add_assign(&mut self, rhs: Vector<T, DIM>) {
        for i in 0..DIM {
            self[i] += rhs[i].clone();
        }
    }
}

impl<T: AddAssign + Clone, const DIM: usize> AddAssign<T> for Vector<T, DIM> {
    fn add_assign(&mut self, rhs: T) {
        for i in 0..DIM {
            self[i] += rhs.clone();
        }
    }
}

impl<T: AddAssign + Add<Output = T> + Clone, const DIM: usize> Add<Vector<T, DIM>>
    for Vector<T, DIM>
{
    type Output = Vector<T, DIM>;
    fn add(self, rhs: Vector<T, DIM>) -> Vector<T, DIM> {
        let mut v = self;
        v += rhs;
        v
    }
}

impl<T: AddAssign + Add<Output = T> + Clone, const DIM: usize> Add<T> for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn add(self, rhs: T) -> Vector<T, DIM> {
        let mut v = self;
        v += rhs;
        v
    }
}

impl<T: SubAssign + Clone, const DIM: usize> SubAssign<Vector<T, DIM>> for Vector<T, DIM> {
    fn sub_assign(&mut self, rhs: Vector<T, DIM>) {
        for i in 0..DIM {
            self[i] -= rhs[i].clone();
        }
    }
}

impl<T: SubAssign + Clone, const DIM: usize> SubAssign<T> for Vector<T, DIM> {
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..DIM {
            self[i] -= rhs.clone();
        }
    }
}

impl<T: SubAssign + Sub<Output = T> + Clone, const DIM: usize> Sub<Vector<T, DIM>>
    for Vector<T, DIM>
{
    type Output = Vector<T, DIM>;
    fn sub(self, rhs: Vector<T, DIM>) -> Vector<T, DIM> {
        let mut v = self;
        v -= rhs;
        v
    }
}

impl<T: SubAssign + Sub<Output = T> + Clone, const DIM: usize> Sub<T> for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn sub(self, rhs: T) -> Vector<T, DIM> {
        let mut v = self;
        v -= rhs;
        v
    }
}

impl<T: MulAssign + Clone, const DIM: usize> MulAssign<Vector<T, DIM>> for Vector<T, DIM> {
    fn mul_assign(&mut self, rhs: Vector<T, DIM>) {
        for i in 0..DIM {
            self[i] *= rhs[i].clone();
        }
    }
}

impl<T: MulAssign + Clone, const DIM: usize> MulAssign<T> for Vector<T, DIM> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..DIM {
            self[i] *= rhs.clone();
        }
    }
}

impl<T: MulAssign + Mul<Output = T> + Clone, const DIM: usize> Mul<Vector<T, DIM>>
    for Vector<T, DIM>
{
    type Output = Vector<T, DIM>;
    fn mul(self, rhs: Vector<T, DIM>) -> Vector<T, DIM> {
        let mut v = self;
        v *= rhs;
        v
    }
}

impl<T: MulAssign + Mul<Output = T> + Clone, const DIM: usize> Mul<T> for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn mul(self, rhs: T) -> Vector<T, DIM> {
        let mut v = self;
        v *= rhs;
        v
    }
}

impl<T: DivAssign + Clone, const DIM: usize> DivAssign<Vector<T, DIM>> for Vector<T, DIM> {
    fn div_assign(&mut self, rhs: Vector<T, DIM>) {
        for i in 0..DIM {
            self[i] /= rhs[i].clone();
        }
    }
}

impl<T: DivAssign + Clone, const DIM: usize> DivAssign<T> for Vector<T, DIM> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..DIM {
            self[i] /= rhs.clone();
        }
    }
}

impl<T: DivAssign + Div<Output = T> + Clone, const DIM: usize> Div<Vector<T, DIM>>
    for Vector<T, DIM>
{
    type Output = Vector<T, DIM>;
    fn div(self, rhs: Vector<T, DIM>) -> Vector<T, DIM> {
        let mut v = self;
        v /= rhs;
        v
    }
}

impl<T: DivAssign + Div<Output = T> + Clone, const DIM: usize> Div<T> for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn div(self, rhs: T) -> Vector<T, DIM> {
        let mut v = self;
        v /= rhs;
        v
    }
}

impl<T: RemAssign + Clone, const DIM: usize> RemAssign<Vector<T, DIM>> for Vector<T, DIM> {
    fn rem_assign(&mut self, rhs: Vector<T, DIM>) {
        for i in 0..DIM {
            self[i] %= rhs[i].clone();
        }
    }
}

impl<T: RemAssign + Clone, const DIM: usize> RemAssign<T> for Vector<T, DIM> {
    fn rem_assign(&mut self, rhs: T) {
        for i in 0..DIM {
            self[i] %= rhs.clone();
        }
    }
}

impl<T: RemAssign + Rem<Output = T> + Clone, const DIM: usize> Rem<Vector<T, DIM>>
    for Vector<T, DIM>
{
    type Output = Vector<T, DIM>;
    fn rem(self, rhs: Vector<T, DIM>) -> Vector<T, DIM> {
        let mut v = self;
        v %= rhs;
        v
    }
}

impl<T: RemAssign + Rem<Output = T> + Clone, const DIM: usize> Rem<T> for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn rem(self, rhs: T) -> Vector<T, DIM> {
        let mut v = self;
        v %= rhs;
        v
    }
}

impl<T: Neg<Output = T> + Clone, const DIM: usize> Neg for Vector<T, DIM> {
    type Output = Vector<T, DIM>;
    fn neg(self) -> Vector<T, DIM> {
        let mut v = self;
        for i in 0..DIM {
            v[i] = -v[i].clone();
        }
        v
    }
}

use dxlib_sys::data::Vector as DxVector;

impl From<Vector3<f32>> for DxVector {
    fn from(v: Vector3<f32>) -> DxVector {
        DxVector {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}


impl From<DxVector> for Vector3<f32> {
    fn from(v: DxVector) -> Vector3<f32> {
        Vector3::from([v.x, v.y, v.z])
    }
}

use dxlib_sys::data::Float4 as DxFloat4;

impl From<Vector4<f32>> for DxFloat4 {
    fn from(v: Vector4<f32>) -> DxFloat4 {
        DxFloat4 {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

impl<T> Vector3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Clone + Copy,
{
    pub fn cross(self, rhs: Vector3<T>) -> Vector3<T> {
        let x = self[1] * rhs[2] - self[2] * rhs[1];
        let y = self[2] * rhs[0] - self[0] * rhs[2];
        let z = self[0] * rhs[1] - self[1] * rhs[0];
        Vector3::from([x, y, z])
    }
}

impl<T> Vector3<T>
where
    T: Copy
        + Clone
        + Neg<Output = T>
        + Mul<Output = T>
        + MulAssign
        + One
        + Add<Output = T>
        + AddAssign
        + Zero,
{
    pub fn up() -> Vector3<T> {
        Self::from([T::zero(), T::one(), T::zero()])
    }

    pub fn down() -> Vector3<T> {
        Self::from([T::zero(), -T::one(), T::zero()])
    }

    pub fn forward() -> Vector3<T> {
        Self::from([T::zero(), T::zero(), T::one()])
    }

    pub fn back() -> Vector3<T> {
        Self::from([T::zero(), T::zero(), -T::one()])
    }

    pub fn right() -> Vector3<T> {
        Self::from([T::one(), T::zero(), T::zero()])
    }

    pub fn left() -> Vector3<T> {
        Self::from([-T::one(), T::zero(), T::zero()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binop() {
        let v = Vector::from([5, 4, 3, 2, 1]);
        assert_eq!(v + 1, Vector::from([6, 5, 4, 3, 2]));
        assert_eq!(v + v, Vector::from([10, 8, 6, 4, 2]));
        assert_eq!(v - v / 2, Vector::from([3, 2, 2, 1, 1]));
        assert_eq!(v * 2 - 5, Vector::from([5, 3, 1, -1, -3]))
    }

    #[test]
    fn test_dot() {
        let v = Vector::from([3.0, 4.0, 12.0]);
        let u = Vector::from([4.0, 4.0, -2.0]);
        assert!((v.dot(u) - 4.0f64).abs() < 1e-6);
        assert!((v.magnitude() - 13.0f64).abs() < 1e-6);
    }

    #[test]
    fn test_cross() {
        let v = Vector::from([1.0, 2.0, 3.0]);
        let u = Vector::from([4.0, 5.0, 6.0]);
        let x = v.cross(u);
        assert!((x[0] - -3.0f64).abs() < 1e-6);
        assert!((x[1] - 6.0f64).abs() < 1e-6);
        assert!((x[2] - -3.0f64).abs() < 1e-6);
    }
}
