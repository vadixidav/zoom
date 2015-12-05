pub mod cartesian3;

extern crate num;
use self::num::Float;
use super::vector::Vector;
use super::particle::{Particle, Quanta};

enum Child<P, N> {
    None,
    Leaf(Box<P>),
    Node(Box<N>),
}

trait BarnesHut<P, V, D>
    where P: Particle<V, D> + Quanta<D>, V: Vector<D>, D: Float
{
    fn interact<F>(interaction: mut F)
        where F: FnMut(&mut P, &);
}
