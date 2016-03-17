extern crate num;
extern crate nalgebra as na;
use self::num::{Float, FromPrimitive};
use super::*;

/// Anything that implements this trait permits its space to be infinitely wrapping.
pub trait Toroid<V> {
    /// Wrap a delta vector between two positions inside of the toroidal space
    fn wrap_delta(&self, delta: V) -> V;

    /// Wrap a position to keep it inside of the space
    fn wrap_position(&self, pos: V) -> V;
}

pub trait Ball<D> {
    fn radius(&self) -> D;

    fn space<V>(&self) -> D
        where V: Vector<D>, D: Float
    {
        V::space_ball(self.radius())
    }
}

/// A Box with a center at origin and one of the corners created by offset
///
/// The box is aligned so that the face normals point along each axis.
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
        self.offset.space_box()
    }
}

impl<D> Toroid<Cartesian1<D>> for Box<Cartesian1<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: Cartesian1<D>) -> Cartesian1<D> {
        Cartesian1{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
        }
    }

    fn wrap_position(&self, pos: Cartesian1<D>) -> Cartesian1<D> {
        self.wrap_delta(pos - self.origin) + self.origin
    }
}

impl<D> Toroid<Cartesian2<D>> for Box<Cartesian2<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: Cartesian2<D>) -> Cartesian2<D> {
        Cartesian2{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
            y: delta.y % (D::from_u32(2u32).unwrap() * self.offset.y),
        }
    }

    fn wrap_position(&self, pos: Cartesian2<D>) -> Cartesian2<D> {
        self.wrap_delta(pos - self.origin) + self.origin
    }
}

impl<D> Toroid<Cartesian3<D>> for Box<Cartesian3<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: Cartesian3<D>) -> Cartesian3<D> {
        Cartesian3{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
            y: delta.y % (D::from_u32(2u32).unwrap() * self.offset.y),
            z: delta.z % (D::from_u32(2u32).unwrap() * self.offset.z),
        }
    }

    fn wrap_position(&self, pos: Cartesian3<D>) -> Cartesian3<D> {
        self.wrap_delta(pos - self.origin) + self.origin
    }
}

impl<D> Toroid<na::Vec1<D>> for Box<na::Vec1<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: na::Vec1<D>) -> na::Vec1<D> {
        na::Vec1{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
        }
    }

    fn wrap_position(&self, pos: na::Vec1<D>) -> na::Vec1<D> {
        self.wrap_delta(pos - self.origin) + self.origin
    }
}

impl<D> Toroid<na::Vec2<D>> for Box<na::Vec2<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: na::Vec2<D>) -> na::Vec2<D> {
        na::Vec2{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
            y: delta.y % (D::from_u32(2u32).unwrap() * self.offset.y),
        }
    }

    fn wrap_position(&self, pos: na::Vec2<D>) -> na::Vec2<D> {
        self.wrap_delta(pos - self.origin) + self.origin
    }
}

impl<D> Toroid<na::Vec3<D>> for Box<na::Vec3<D>>
    where D: Float + FromPrimitive
{
    fn wrap_delta(&self, delta: na::Vec3<D>) -> na::Vec3<D> {
        na::Vec3{
            x: delta.x % (D::from_u32(2u32).unwrap() * self.offset.x),
            y: delta.y % (D::from_u32(2u32).unwrap() * self.offset.y),
            z: delta.z % (D::from_u32(2u32).unwrap() * self.offset.z),
        }
    }

    fn wrap_position(&self, pos: na::Vec3<D>) -> na::Vec3<D> {
        self.wrap_delta(pos - self.origin) + self.origin
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
