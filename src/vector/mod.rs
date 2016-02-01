//!The vector module contains primitive objects and traits for performing vector geometry.

pub mod cartesian1;
pub mod cartesian2;
pub mod cartesian3;
pub mod space;
pub use self::cartesian1::*;
pub use self::cartesian2::*;
pub use self::cartesian3::*;
pub use self::space::*;

extern crate num;
extern crate nalgebra as na;
use self::num::{Float, Zero, FromPrimitive};
use std::ops::{Add, Sub, Neg, Mul, Div};
use std::f64::consts::PI;

///Trait that implements all the functions necessary for any n-dimensional particle.
pub trait Vector<D>: Sized + Clone + Copy + Zero + Add<Self, Output=Self> + Sub<Self, Output=Self> + Neg<Output=Self> +
    Mul<D, Output=Self> + Div<D, Output=Self>
    where D: Float
{
    ///Returns the space contained by an n-sphere with edge displacement d in the dimensional system of this vector
    fn space_ball(d: D) -> D;

    ///Returns the result of the cos of the angle between two vectors multiplied by their magnitudes
    fn dot(&lhs: &Self, rhs: &Self) -> D;

    ///Returns the space contained by the vector relative to the origin forming a box
    fn space_box(&self) -> D;

    ///Returns the length of a vector
    fn displacement(&self) -> D;

    ///Returns the squared length of a vector; this is more efficient than displacement() for cartesian vectors
    fn displacement_squared(&self) -> D {
        self.displacement().powi(2)
    }

    ///Returns a vector in the same direction as this one, but with length 1
    fn normalized(&self) -> Self {
        *self / self.displacement()
    }

    ///Make this vector normalized().
    fn normalize(&mut self) {
        *self = self.normalized();
    }
}

#[test]
fn dot_vector() {
    let a = Cartesian2::new(0.3, 0.5);
    let _b = Cartesian2::dot(&a, &Cartesian2::new(1.0, 0.5));
}

///CrossVector is a Vector that has dimensions such that the cross product can be computed.
pub trait CrossVector {
    fn cross(lhs: &Self, rhs: &Self) -> Self;
}

#[test]
fn cross_vector() {
    let a = Cartesian3::new(0.3, 0.5, 1.0);
    let _b = Cartesian3::cross(&a, &Cartesian3::new(1.0, 0.5, -2.0));
}

impl<D> Vector<D> for na::Vec1<D>
    where D: Float + FromPrimitive
{
    fn space_ball(d: D) -> D {
        D::from_u32(2u32).unwrap() * d
    }
    fn dot(&lhs: &Self, rhs: &Self) -> D {
        lhs.x * rhs.x
    }
    fn space_box(&self) -> D {
        self.x
    }
    fn displacement(&self) -> D {
        self.x
    }
}

impl<D> Vector<D> for na::Vec2<D>
    where D: Float + FromPrimitive
{
    fn space_ball(d: D) -> D {
        D::from_f64(PI).unwrap() * d * d
    }
    fn dot(&lhs: &Self, rhs: &Self) -> D {
        lhs.x * rhs.x + lhs.y * rhs.y
    }
    fn space_box(&self) -> D {
        self.x * self.y
    }
    fn displacement(&self) -> D {
        self.displacement_squared().sqrt()
    }
    fn displacement_squared(&self) -> D {
        self.x * self.x + self.y * self.y
    }
}

impl<D> Vector<D> for na::Vec3<D>
    where D: Float + FromPrimitive
{
    fn space_ball(d: D) -> D {
        d * d * d * D::from_f64(4.0 / 3.0 * PI).unwrap()
    }
    fn dot(&lhs: &Self, rhs: &Self) -> D {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
    fn space_box(&self) -> D {
        self.x * self.y * self.z
    }
    fn displacement(&self) -> D {
        self.displacement_squared().sqrt()
    }
    fn displacement_squared(&self) -> D {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<D> CrossVector for na::Vec3<D>
    where D: Float
{
    fn cross(lhs: &Self, rhs: &Self) -> Self {
        na::Vec3{
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}
