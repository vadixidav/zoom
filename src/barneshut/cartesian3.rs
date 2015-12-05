extern crate num;
use self::num::Float;
use super::super::vector::Vector;
use super::super::particle::{Particle, Quanta};
use super::{BarnesHut, Child};

struct Node<P, V, D>
    where P: Particle<V, D> + Quanta<D>, V: Vector<D>, D: Float
{
    origin: V,
    offset: V,
    branches:
}

struct Tree {
    
}
