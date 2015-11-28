pub mod cartesian1;
pub use self::cartesian1::*;

extern crate num;
use self::num::Float;
use std::ops::{Add, Sub, Neg, Mul, Div};

//Trait that implements all the functions necessary for any n-dimensional particle
pub trait Vector<D>: Sized + Clone + Copy + Add<Self, Output=Self> + Sub<Self, Output=Self> + Neg<Output=Self> +
    Mul<D, Output=Self> + Div<D, Output=Self>
    where D: Float
{
    fn displacement(&self) -> D;
    fn normalized(&self) -> Self {
        *self / self.displacement()
    }
    fn normalize(&mut self) {
        *self = self.normalized();
    }
}
