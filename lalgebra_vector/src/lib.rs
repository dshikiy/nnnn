use std::ops::Add;

pub trait Scalar:
    Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Sized
    + Clone
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar {
    ($($t:ty, $z:expr, $o:expr)*) => {
        $(
            impl Scalar for $t {
                type Item = $t;
                fn zero() -> Self::Item { $z }
                fn one() -> Self::Item { $o }
            }
        )*
    }
}

impl_scalar!(u32, 0, 1  i32, 0, 1  u64, 0, 1  i64, 0, 1  f32, 0.0, 1.0  f64, 0.0, 1.0);

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar<Item = T>> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        
        let res: Vec<T> = self.0.into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
            
        Some(Vector(res))
    }
}

impl<T: Scalar<Item = T>> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        
        let mut sum = T::zero();
        for (a, b) in self.0.into_iter().zip(rhs.0.into_iter()) {
            sum = sum + (a * b);
        }
        
        Some(sum)
    }
}