extern crate num;

use self::num::Float;
use super::super::{Vector, Cartesian3, Position, Box};
use super::{Child, SpatialTree};
use std::ops::{DerefMut, Deref};
use std::{mem, boxed};
use std::cell::UnsafeCell;

struct Node<O, V> {
    //The volume of the tree
    location: Box<V>,
    //Each of the octants in the octree
    branches: [Child<O, Node<O, V>>; 8],
}

pub struct Tree<O, V> {
    //The root node of the tree which encompases some space specified in the node
    root: Node<O, V>,
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

impl<'a, O, V> Iterator for Iter<'a, O, V> {
    type Item = &'a O;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.stack.pop();
        match result {
            Some(info) => {
                if info.branch == 8 {
                    self.next()
                } else {
                    self.stack.push(IterInfo{node: info.node, branch: info.branch + 1});
                    match info.node.branches[info.branch] {
                        Child::Leaf(ref l) => Some(unsafe{&*l.get()}.deref()),
                        Child::Node(ref n) => {
                            self.stack.push(IterInfo{node: n.deref(), branch: 0});
                            self.next()
                        },
                        Child::None => None,
                    }
                }
            },
            //Continue to return None so long as the iterator is empty
            None => None,
        }
    }
}

pub struct IterMut<'a, O, V>
    where O: 'a, V: 'a
{
    //The stack of where we are iterating, once this is empty we return None
    stack: Vec<IterInfo<'a, O, V>>,
}

impl<'a, O, V> Iterator for IterMut<'a, O, V> {
    type Item = &'a mut O;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.stack.pop();
        match result {
            Some(info) => {
                if info.branch == 8 {
                    self.next()
                } else {
                    self.stack.push(IterInfo{node: info.node, branch: info.branch + 1});
                    match info.node.branches[info.branch] {
                        Child::Leaf(ref l) => Some(unsafe{&mut *l.get()}.deref_mut()),
                        Child::Node(ref n) => {
                            self.stack.push(IterInfo{node: n.deref(), branch: 0});
                            self.next()
                        },
                        Child::None => None,
                    }
                }
            },
            //Continue to return None so long as the iterator is empty
            None => None,
        }
    }
}

impl<'a, O, V, D> SpatialTree<'a, O, V, D> for Tree<O, V>
    where O: 'a + Position<V>, V: 'a + Vector<D>, D: 'a + Float
{
    type Iter = Iter<'a, O, V>;
    type IterMut = IterMut<'a, O, V>;

    fn iter(&'a self) -> Self::Iter {
        Iter{stack: vec![IterInfo{node: &self.root, branch: 0}]}
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        IterMut{stack: vec![IterInfo{node: &self.root, branch: 0}]}
    }

    fn insert(&mut self, obj: O) {
    }
}
