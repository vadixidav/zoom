extern crate zoom;
use zoom::{Vector, CrossVector, Cartesian2, Cartesian3};

#[test]
fn dot_vector() {
    let a = Cartesian2::new(0.3, 0.5);
    let _b = Cartesian2::dot(&a, &Cartesian2::new(1.0, 0.5));
}

#[test]
fn cross_vector() {
    let a = Cartesian3::new(0.3, 0.5, 1.0);
    let _b = Cartesian3::cross(&a, &Cartesian3::new(1.0, 0.5, -2.0));
}
