pub mod cartesian1;
pub mod cartesian2;
pub mod cartesian3;
pub use self::cartesian1::*;
pub use self::cartesian2::*;
pub use self::cartesian3::*;

extern crate num;
use self::num::{Float, Zero};
use std::ops::{Add, Sub, Neg, Mul, Div};

//Trait that implements all the functions necessary for any n-dimensional particle
pub trait Vector<D>: Sized + Clone + Copy + Zero + Add<Self, Output=Self> + Sub<Self, Output=Self> + Neg<Output=Self> +
    Mul<D, Output=Self> + Div<D, Output=Self>
    where D: Float
{
    //Returns the result of the cos of the angle between two vectors multiplied by their magnitudes
    fn dot(&lhs: &Self, rhs: &Self) -> D;

    //Returns the space contained by the vector relative to the origin
    fn space(&self) -> D;

    //Returns the length of a vector
    fn displacement(&self) -> D;

    //Returns the squared length of a vector; this is more efficient than displacement() for cartesian vectors
    fn displacement_squared(&self) -> D {
        self.displacement().powi(2)
    }

    //Return a vector in the same direction as this one, but with length 1
    fn normalized(&self) -> Self {
        *self / self.displacement()
    }

    //Make this vector normalized()
    fn normalize(&mut self) {
        *self = self.normalized();
    }
}

//CrossVector is a Vector that has dimensions such that the cross product can be computed
pub trait CrossVector<D>: Vector<D>
    where D: Float
{
    fn cross(lhs: &Self, rhs: &Self) -> Self;
}

pub struct Box<V> {
    pub origin: V,
    pub offset: V,
}

impl<V> Box<V> {
    pub fn new(origin: V, offset: V) -> Self {
        Box{
            origin: origin,
            offset: offset,
        }
    }

    //Compute the amount of space contained in the box
    pub fn space<D>(&self) -> D
        where V: Vector<D>, D: Float
    {
        self.offset.space()
    }
}

impl<V> Clone for Box<V>
    where V: Clone
{
    fn clone(&self) -> Self {
        Box{
            origin: self.origin.clone(),
            offset: self.offset.clone(),
        }
    }
}
