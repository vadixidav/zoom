extern crate num;
use super::{Vector, CrossVector};
use self::num::Float;
use std::ops::{Add, Sub, Neg, Mul, Div};

pub struct Cartesian3<D> {
    x: D,
    y: D,
    z: D,
}

impl<D> Cartesian3<D> where D: Copy {
    pub fn new(x: D, y: D, z: D) -> Self {
        Cartesian3{x: x, y: y, z: z}
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
    fn dot(&self, rhs: &Self) -> D {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    fn displacement(&self) -> D {
        self.displacement_squared().sqrt()
    }
    fn displacement_squared(&self) -> D {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl <D> CrossVector<D> for Cartesian3<D>
    where D: Float
{
    fn cross(&self, rhs: &Self) -> Self {
        Cartesian3{
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
