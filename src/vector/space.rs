extern crate num;
use self::num::Float;
use super::*;

pub trait Ball<D> {
    fn radius(&self) -> D;

    fn space<V>(&self) -> D
        where V: Vector<D>, D: Float
    {
        V::space_ball(self.radius())
    }
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
        self.offset.space_box()
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
