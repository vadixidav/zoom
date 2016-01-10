extern crate num;

use self::num::Float;
use super::super::{Vector, Cartesian3, Position, Box};
use super::{Child, SpatialTree};
use std::ops::{DerefMut, Deref};

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
    type Item = &'a mut O;
    fn next(&mut self) -> Option<&'a mut O> {
        let result = self.stack.pop();
        match result {
            Some(info) => {
                if info.branch == 7 {
                    self.next()
                } else {
                    match info.node.branches[info.branch] {
                        Child::Leaf(ref mut l) => Some(l.deref_mut()),
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
    type RadiusIter = Iter<'a, O, V>;
    type RadiusSquaredIter = Iter<'a, O, V>;
    type CubeIter = Iter<'a, O, V>;
    type CornerIter = Iter<'a, O, V>;

    fn iter(&'a mut self) -> Self::Iter {
        Iter{stack: vec![IterInfo{node: &mut self.root, branch: 0}]}
    }

    fn insert(&mut self, obj: O) {
    }
}
