pub mod cartesian3;

extern crate num;

use std::cell::UnsafeCell;

enum Child<O, N> {
    None,
    Leaf(UnsafeCell<Box<O>>),
    Node(Box<N>),
}

/*
SpatialTree implementors store objects with positions such that they can be efficiently retrieved based on spatial
locality and position.

O: Object to be stored
V: Vector
D: Displacement type
*/
pub trait SpatialTree<'a, O, V, D>: 'a
    where O: 'a
{
    type Iter: Iterator<Item=&'a O>;
    type IterMut: Iterator<Item=&'a mut O>;

    //Iterate over all items in the tree.
    fn iter(&'a self) -> Self::Iter;

    //Iterate over all items in the tree mutably.
    fn iter_mut(&'a mut self) -> Self::IterMut;

    //Insert an item into the tree
    fn insert(&mut self, obj: O);
}
