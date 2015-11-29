extern crate num;
use super::Vector;
use self::num::Float;
use std::ops::{Add, Sub, Neg, Mul, Div};

pub struct Cartesian2<D> {
    x: D,
    y: D,
}

impl<D> Cartesian2<D> where D: Copy {
    pub fn new(x: D, y: D) -> Self {
        Cartesian2{x: x, y: y}
    }
}

impl<D> Cartesian2<D> where D: Float {
    fn displacement_squared(&self) -> D {
        self.x * self.x + self.y * self.y
    }
}

impl<D> Clone for Cartesian2<D> where D: Copy {
    fn clone(&self) -> Self {
        Cartesian2{x: self.x, y: self.y}
    }
}

impl<D> Copy for Cartesian2<D> where D: Copy {}

impl<D> Add for Cartesian2<D>
    where D: Float
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Cartesian2{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<D> Sub for Cartesian2<D>
    where D: Float
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Cartesian2{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<D> Mul<D> for Cartesian2<D>
    where D: Float
{
    type Output = Self;
    fn mul(self, rhs: D) -> Self {
        Cartesian2{x: self.x * rhs, y: self.y * rhs}
    }
}

impl<D> Div<D> for Cartesian2<D>
    where D: Float
{
    type Output = Self;
    fn div(self, rhs: D) -> Self {
        Cartesian2{x: self.x / rhs, y: self.y / rhs}
    }
}

impl<D> Neg for Cartesian2<D>
    where D: Float
{
    type Output = Self;
    fn neg(self) -> Self {
        Cartesian2{x: -self.x, y: -self.y}
    }
}

impl<D> Vector<D> for Cartesian2<D>
    where D: Float
{
    fn dot(&self, rhs: &Self) -> D {
        self.x * rhs.x + self.y * rhs.y
    }
    fn displacement(&self) -> D {
        self.displacement_squared().sqrt()
    }
}
