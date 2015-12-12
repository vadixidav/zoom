extern crate num;

use self::num::Float;
use super::super::vector;
use super::super::vector::{Vector, Cartesian3};
use super::super::particle::Position;
use super::{Child, SpatialTree};

struct Node<O, V> {
    //The volume of the tree
    location: vector::Box<V>,
    //Each of the octants in the octree
    branches: [Child<O, Node<O, V>>; 8],
}

pub struct Tree<O, V> {
    //The root node of the tree which encompases some space specified in the node
    root: Child<O, Node<O, V>>,
}

struct IterInfo<'a, O, V>
    where O: 'a, V: 'a
{
    //The node of this part of the iteration
    node: &'a Node<O, V>,
    //Count across each 8 octants
    branch: usize,
}

pub struct Iter<'a, O, V>
    where O: 'a, V: 'a
{
    //The stack of where we are iterating, once this is empty we return None
    stack: Vec<IterInfo<'a, O, V>>,
}
/*
impl<'a, O, V, D> SpatialTree<'a, O, V, D> for Tree<O, V> {

}*/
