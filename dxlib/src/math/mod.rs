pub mod matrix;
pub mod vector;

pub trait DotProduct<T> {
    type Output;
    fn dot(self, other: T) -> Self::Output;
}
