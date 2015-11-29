pub mod cartesian1;
pub mod cartesian2;
pub mod cartesian3;
pub use self::cartesian1::*;
pub use self::cartesian2::*;
pub use self::cartesian3::*;

extern crate num;
use self::num::Float;
use std::ops::{Add, Sub, Neg, Mul, Div};

//Trait that implements all the functions necessary for any n-dimensional particle
pub trait Vector<D>: Sized + Clone + Copy + Add<Self, Output=Self> + Sub<Self, Output=Self> + Neg<Output=Self> +
    Mul<D, Output=Self> + Div<D, Output=Self>
    where D: Float
{
    fn dot(&self, rhs: &Self) -> D;
    fn displacement(&self) -> D;
    fn displacement_squared(&self) -> D {
        self.displacement().powi(2)
    }
    fn normalized(&self) -> Self {
        *self / self.displacement()
    }
    fn normalize(&mut self) {
        *self = self.normalized();
    }
}

//CrossVector is a Vector that is of high enough dimension to have a cross product
pub trait CrossVector<D>: Vector<D>
    where D: Float
{
    fn cross(&self, rhs: &Self) -> Self;
}
