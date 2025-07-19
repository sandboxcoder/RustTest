mod vector;

use vector::Vector;

fn main() {
    let a: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
    let b: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
    println!("dot prod a*b ({})", a.dot(b));
}
