use num_traits::{One, Zero};
use std::{ops::*};

use crate::vector::Vector;

pub type SqMatrix<T, const DIM: usize> = Matrix<T, DIM, DIM>;

pub trait DotProduct<T> {
    type Output;
    fn dot(&self, other: T) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T, const ROW: usize, const COL: usize>([[T; COL]; ROW]);

impl<T, const A: usize, const B: usize, const C: usize> DotProduct<Matrix<T, B, C>>
    for Matrix<T, A, B>
where
    T: Zero + Copy + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T, A, C>;
    fn dot(&self, other: Matrix<T, B, C>) -> Matrix<T, A, C> {
        let mut v = Matrix::zero();
        for a in 0..A {
            for c in 0..C {
                let mut cell = T::zero();
                for b in 0..B {
                    cell += self[a][b] * other[b][c];
                }
                v[a][c] = cell;
            }
        }
        v
    }
}

impl<T, const ROW: usize, const COL: usize> DotProduct<Vector<T, COL>> for Matrix<T, ROW, COL>
where
    T: Zero + Copy + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    type Output = Vector<T, ROW>;
    fn dot(&self, other: Vector<T, COL>) -> Vector<T, ROW> {
        let mut v = Vector::zero();
        for a in 0..ROW {
            let mut cell = T::zero();
            for b in 0..COL {
                cell += self[a][b] * other[b];
            }
            v[a] = cell;
        }
        v
    }
}

impl<T, const ROW: usize, const COL: usize> Matrix<T, ROW, COL>
where
    T: Zero + Copy + AddAssign + Add<Output=T>
{
    pub fn transpose(&self) -> Matrix<T, COL, ROW> {
        let mut v = Matrix::zero();
        for i in 0..ROW {
            for j in 0..COL {
                v[j][i] = self[i][j];
            }
        }
        v
    }
}


impl<T, const ROW: usize, const COL: usize> From<[[T; COL]; ROW]> for Matrix<T, ROW, COL>
where
    T: Copy,
{
    fn from(v: [[T; COL]; ROW]) -> Self {
        Self(v)
    }
}

impl<T, const ROW: usize, const COL: usize> Default for Matrix<T, ROW, COL>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self([[T::default(); COL]; ROW])
    }
}

impl<T, const ROW: usize, const COL: usize> Zero for Matrix<T, ROW, COL>
where
    T: Zero + Copy + AddAssign + Add<Output = T>,
{
    fn is_zero(&self) -> bool {
        let mut v = true;
        for i in 0..ROW {
            for j in 0..COL {
                v &= self[i][j].is_zero();
            }
        }
        v
    }
    fn zero() -> Self {
        Matrix::from([[T::zero(); COL]; ROW])
    }
}

impl<T, const ROW: usize, const COL: usize> One for Matrix<T, ROW, COL>
where
    T: One + Copy + MulAssign + Mul<Output = T> + PartialEq,
{
    fn is_one(&self) -> bool {
        let mut v = true;
        for i in 0..ROW {
            for j in 0..COL {
                v &= self[i][j].is_one();
            }
        }
        v
    }
    fn one() -> Self {
        Matrix::from([[T::one(); COL]; ROW])
    }
}

impl<T, const ROW: usize, const COL: usize> Index<usize> for Matrix<T, ROW, COL> {
    type Output = [T; COL];
    fn index(&self, index: usize) -> &[T; COL] {
        &self.0[index]
    }
}

impl<T, const ROW: usize, const COL: usize> IndexMut<usize> for Matrix<T, ROW, COL> {
    fn index_mut(&mut self, index: usize) -> &mut [T; COL] {
        &mut self.0[index]
    }
}

impl<T: AddAssign + Clone, const ROW: usize, const COL: usize> AddAssign<Matrix<T, ROW, COL>>
    for Matrix<T, ROW, COL>
{
    fn add_assign(&mut self, rhs: Matrix<T, ROW, COL>) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] += rhs[i][j].clone();
            }
        }
    }
}

impl<T: AddAssign + Clone, const ROW: usize, const COL: usize> AddAssign<T>
    for Matrix<T, ROW, COL>
{
    fn add_assign(&mut self, rhs: T) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] += rhs.clone();
            }
        }
    }
}

impl<T: AddAssign + Add<Output = T> + Clone, const ROW: usize, const COL: usize>
    Add<Matrix<T, ROW, COL>> for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn add(self, rhs: Matrix<T, ROW, COL>) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v += rhs;
        v
    }
}

impl<T: AddAssign + Add<Output = T> + Clone, const ROW: usize, const COL: usize> Add<T>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn add(self, rhs: T) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v += rhs;
        v
    }
}

impl<T: SubAssign + Clone, const ROW: usize, const COL: usize> SubAssign<Matrix<T, ROW, COL>>
    for Matrix<T, ROW, COL>
{
    fn sub_assign(&mut self, rhs: Matrix<T, ROW, COL>) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] -= rhs[i][j].clone();
            }
        }
    }
}

impl<T: SubAssign + Clone, const ROW: usize, const COL: usize> SubAssign<T>
    for Matrix<T, ROW, COL>
{
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] -= rhs.clone();
            }
        }
    }
}

impl<T: SubAssign + Sub<Output = T> + Clone, const ROW: usize, const COL: usize>
    Sub<Matrix<T, ROW, COL>> for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn sub(self, rhs: Matrix<T, ROW, COL>) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v -= rhs;
        v
    }
}

impl<T: SubAssign + Sub<Output = T> + Clone, const ROW: usize, const COL: usize> Sub<T>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn sub(self, rhs: T) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v -= rhs;
        v
    }
}

impl<T: MulAssign + Clone, const ROW: usize, const COL: usize> MulAssign<Matrix<T, ROW, COL>>
    for Matrix<T, ROW, COL>
{
    fn mul_assign(&mut self, rhs: Matrix<T, ROW, COL>) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] *= rhs[i][j].clone();
            }
        }
    }
}

impl<T: MulAssign + Clone, const ROW: usize, const COL: usize> MulAssign<T>
    for Matrix<T, ROW, COL>
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] *= rhs.clone();
            }
        }
    }
}

impl<T: MulAssign + Mul<Output = T> + Clone, const ROW: usize, const COL: usize>
    Mul<Matrix<T, ROW, COL>> for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn mul(self, rhs: Matrix<T, ROW, COL>) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v *= rhs;
        v
    }
}

impl<T: MulAssign + Mul<Output = T> + Clone, const ROW: usize, const COL: usize> Mul<T>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn mul(self, rhs: T) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v *= rhs;
        v
    }
}

impl<T: DivAssign + Clone, const ROW: usize, const COL: usize> DivAssign<Matrix<T, ROW, COL>>
    for Matrix<T, ROW, COL>
{
    fn div_assign(&mut self, rhs: Matrix<T, ROW, COL>) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] /= rhs[i][j].clone();
            }
        }
    }
}

impl<T: DivAssign + Clone, const ROW: usize, const COL: usize> DivAssign<T>
    for Matrix<T, ROW, COL>
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] /= rhs.clone();
            }
        }
    }
}

impl<T: DivAssign + Div<Output = T> + Clone, const ROW: usize, const COL: usize>
    Div<Matrix<T, ROW, COL>> for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn div(self, rhs: Matrix<T, ROW, COL>) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v /= rhs;
        v
    }
}

impl<T: DivAssign + Div<Output = T> + Clone, const ROW: usize, const COL: usize> Div<T>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn div(self, rhs: T) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v /= rhs;
        v
    }
}

impl<T: RemAssign + Clone, const ROW: usize, const COL: usize> RemAssign<Matrix<T, ROW, COL>>
    for Matrix<T, ROW, COL>
{
    fn rem_assign(&mut self, rhs: Matrix<T, ROW, COL>) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] %= rhs[i][j].clone();
            }
        }
    }
}

impl<T: RemAssign + Clone, const ROW: usize, const COL: usize> RemAssign<T>
    for Matrix<T, ROW, COL>
{
    fn rem_assign(&mut self, rhs: T) {
        for i in 0..ROW {
            for j in 0..COL {
                self[i][j] %= rhs.clone();
            }
        }
    }
}

impl<T: RemAssign + Rem<Output = T> + Clone, const ROW: usize, const COL: usize>
    Rem<Matrix<T, ROW, COL>> for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn rem(self, rhs: Matrix<T, ROW, COL>) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v %= rhs;
        v
    }
}

impl<T: RemAssign + Rem<Output = T> + Clone, const ROW: usize, const COL: usize> Rem<T>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, COL>;
    fn rem(self, rhs: T) -> Matrix<T, ROW, COL> {
        let mut v = self.clone();
        v %= rhs;
        v
    }
}

pub struct T;

impl<U, const ROW: usize, const COL: usize> BitXor<T> for Matrix<U, ROW, COL>
where
    U: Zero + Copy + AddAssign + Add<Output=U>
{
    type Output = Matrix<U, COL, ROW>;
    fn bitxor(self, _: T) -> Matrix<U, COL, ROW> {
        self.transpose()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let m1 = Matrix::from([[1, 2], [3, 4]]);
        let m2 = Matrix::from([[5, 6], [7, 8]]);
        assert_eq!(m1.dot(m2), Matrix::from([[19, 22], [43, 50]]));
    }
    
    #[test]
    fn test_transpose() {
        let m = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8]]);
        let t = Matrix::from([[1, 5], [2, 6], [3, 7], [4, 8]]);
        assert_eq!(m.transpose(), t);
        assert_eq!(m ^ T, t);
    }
}
