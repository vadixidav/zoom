extern crate num;
use super::{Vector, CrossVector};
use self::num::{Float, Zero};
use std::ops::{Add, Sub, Neg, Mul, Div};

pub struct Cartesian3<D> {
    pub x: D,
    pub y: D,
    pub z: D,
}

impl<D> Cartesian3<D> where D: Copy {
    pub fn new(x: D, y: D, z: D) -> Self {
        Cartesian3{x: x, y: y, z: z}
    }
}

impl<D> Zero for Cartesian3<D>
    where D: Float
{
    fn zero() -> Self {
        Cartesian3{x: D::zero(), y: D::zero(), z: D::zero()}
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<D> Clone for Cartesian3<D> where D: Copy {
    fn clone(&self) -> Self {
        Cartesian3{x: self.x, y: self.y, z: self.z}
    }
}

impl<D> Copy for Cartesian3<D> where D: Copy {}

impl<D> Add for Cartesian3<D>
    where D: Float
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Cartesian3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<D> Sub for Cartesian3<D>
    where D: Float
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Cartesian3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<D> Mul<D> for Cartesian3<D>
    where D: Float
{
    type Output = Self;
    fn mul(self, rhs: D) -> Self {
        Cartesian3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<D> Div<D> for Cartesian3<D>
    where D: Float
{
    type Output = Self;
    fn div(self, rhs: D) -> Self {
        Cartesian3{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl<D> Neg for Cartesian3<D>
    where D: Float
{
    type Output = Self;
    fn neg(self) -> Self {
        Cartesian3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl<D> Vector<D> for Cartesian3<D>
    where D: Float
{
    fn dot(&lhs: &Self, rhs: &Self) -> D {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
    fn space(&self) -> D {
        self.x * self.y
    }
    fn displacement(&self) -> D {
        self.displacement_squared().sqrt()
    }
    fn displacement_squared(&self) -> D {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl <D> CrossVector for Cartesian3<D>
    where D: Float
{
    fn cross(lhs: &Self, rhs: &Self) -> Self {
        Cartesian3{
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}
