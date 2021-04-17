use std::ops::{Index, IndexMut};

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

pub struct Vector<T, const DIM: usize>([T; DIM]);

impl<T, const DIM: usize> From<&[T; DIM]> for Vector<T, DIM>
where
    T: Clone,
{
    fn from(v: &[T; DIM]) -> Self {
        Self(v.clone())
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
