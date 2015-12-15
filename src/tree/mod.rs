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

    //Iterate over all items within a certain distance of a position given by radius.
    //Prefer radius_squared for cartesians, for which it is the most efficient solution.
    fn iter_radius(&'a mut self, center: V, radius: D) -> Self::RadiusIter;

    //Iterate over all items within a certain distance of a position given by radius_squared.
    fn iter_radius_squared(&'a mut self, center: V, radius_squared: D) -> Self::RadiusSquaredIter;

    //Iterate over all items within a cube at a center position and with distance half_side as half the side length.
    fn iter_cube_center(&'a mut self, center: V, half_side: D) -> Self::CubeIter;

    //Iterate over all items within a cube at a corner position center and with side length in the positive direction.
    fn iter_cube_corner(&'a mut self, corner: V, side: D) -> Self::CornerIter;

    //Insert an item into the tree
    fn insert(&mut self, obj: O);
}
