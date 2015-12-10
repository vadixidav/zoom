extern crate num;
use self::num::Float;
use super::super::vector;
use super::super::vector::{Vector, Cartesian3};
use super::super::particle::Position;
use super::Child;

struct Node<O, V, D>
    where V: Vector<D>, D: Float
{
    branches: [Child<O, Node<O, V, D>>; 8],
    location: vector::Box<V, D>,
}

pub struct Tree<O, V, D>
    where V: Vector<D>, D: Float
{
    offset: V,
    root: Child<O, Node<O, V, D>>,
}

struct IterInfo<'a, O, V, D>
    where O: 'a, V: 'a + Vector<D>, D: 'a + Float
{
    //The node of this part of the iteration
    node: &'a Node<O, V, D>,
    branch: usize,
    //This is the offset of the present Node
    offset: V,
}

pub struct Iter<'a, O, V, D>
    where O: 'a + Position<V, D>, V: 'a + Vector<D>, D: 'a + Float
{
    tree: &'a mut Tree<O, V, D>,
    stack: Vec<IterInfo<'a, O, V, D>>,
}
