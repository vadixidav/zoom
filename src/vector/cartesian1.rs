extern crate num;
use super::Vector;
use self::num::{Float, Zero};
use std::ops::{Add, Sub, Neg, Mul, Div};

pub struct Cartesian1<D> {
    pub x: D
}

impl<D> Cartesian1<D> where D: Copy {
    pub fn new(x: D) -> Self {
        Cartesian1{x: x}
    }
}

impl<D> Zero for Cartesian1<D>
    where D: Float
{
    fn zero() -> Self {
        Cartesian1{x: D::zero()}
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero()
    }
}

impl<D> Clone for Cartesian1<D> where D: Copy {
    fn clone(&self) -> Self {
        Cartesian1{x: self.x}
    }
}

impl<D> Copy for Cartesian1<D> where D: Copy {}

impl<D> Add for Cartesian1<D>
    where D: Float
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Cartesian1{x: self.x + rhs.x}
    }
}

impl<D> Sub for Cartesian1<D>
    where D: Float
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Cartesian1{x: self.x - rhs.x}
    }
}

impl<D> Mul<D> for Cartesian1<D>
    where D: Float
{
    type Output = Self;
    fn mul(self, rhs: D) -> Self {
        Cartesian1{x: self.x * rhs}
    }
}

impl<D> Div<D> for Cartesian1<D>
    where D: Float
{
    type Output = Self;
    fn div(self, rhs: D) -> Self {
        Cartesian1{x: self.x / rhs}
    }
}

impl<D> Neg for Cartesian1<D>
    where D: Float
{
    type Output = Self;
    fn neg(self) -> Self {
        Cartesian1{x: -self.x}
    }
}

impl<D> Vector<D> for Cartesian1<D>
    where D: Float
{
    fn dot(&lhs: &Self, rhs: &Self) -> D {
        lhs.x * rhs.x
    }
    fn displacement(&self) -> D {
        self.x
    }
}
