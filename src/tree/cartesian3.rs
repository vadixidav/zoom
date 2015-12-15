extern crate num;

use self::num::Float;
use super::super::{Vector, Cartesian3, Position, Box};
use super::{Child, SpatialTree};

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
        None
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

    fn iter_radius(&'a mut self, center: V, radius: D) -> Self::RadiusIter {
        self.iter_radius_squared(center, radius * radius)
    }

    fn iter_radius_squared(&'a mut self, center: V, radius_squared: D) -> Self::RadiusSquaredIter {
        Iter{stack: Vec::new()}
    }

    fn iter_cube_center(&'a mut self, center: V, half_side: D) -> Self::CubeIter {
        Iter{stack: Vec::new()}
    }

    fn iter_cube_corner(&'a mut self, corner: V, side: D) -> Self::CornerIter {
        Iter{stack: Vec::new()}
    }

    fn insert(&mut self, obj: O) {
    }
}
