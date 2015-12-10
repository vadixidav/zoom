extern crate num;
use self::num::Float;
use super::super::vector::Vector;
use super::super::vector::Cartesian3;
use super::super::particle::Position;
use super::Child;

struct Node<O, V, D>
    where V: Vector<D>, D: Float
{
    branches: [Child<O, Node<O, V, D>>; 8],
    center: V,
    offset: V,
}

pub struct Tree<O, V, D>
    where V: Vector<D>, D: Float
{
    offset: V,
    root: Child<O, Node<O>>,
}

struct IterInfo<O, V, D>
    where V: Vector<D>, D: Float
{
    //The node of this part of the iteration
    node: Node<O>,
    branch: usize,
    //This is the offset of the present Node
    offset: V,
}

pub struct Iter<'a, O, V, D>
    where O: 'a + Position<V, D>, V: 'a + Vector<D>, D: 'a + Float
{
    tree: &'a mut Tree<O, V, D>,

}
