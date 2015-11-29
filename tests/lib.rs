extern crate zoom;
use zoom::Cartesian2;
use zoom::Vector;

#[test]
fn dot_vector() {
    let a = Cartesian2::new(0.3, 0.5);
    let _b = a.dot(&Cartesian2::new(1.0, 0.5));
}
