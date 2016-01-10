pub mod cartesian3;

extern crate num;

enum Child<O, N> {
    None,
    Leaf(Box<O>),
    Node(Box<N>),
}

/*
SpatialTree implementors store objects with positions such that they can be efficiently retrieved based on spatial
locality and position.

O: Object to be stored
V: Vector
D: Displacement type
*/
trait SpatialTree<'a, O, V, D>: 'a
    where O: 'a
{
    type Iter: Iterator<Item=&'a mut O>;
    type RadiusIter: Iterator<Item=&'a mut O>;
    type RadiusSquaredIter: Iterator<Item=&'a mut O>;
    type CubeIter: Iterator<Item=&'a mut O>;
    type CornerIter: Iterator<Item=&'a mut O>;

    //Iterate over all items in the tree.
    fn iter(&'a mut self) -> Self::Iter;

    //Insert an item into the tree
    fn insert(&mut self, obj: O);
}
