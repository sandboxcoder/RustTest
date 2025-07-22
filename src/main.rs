mod vector;

use vector::Vector3;

fn main() {
    let a: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    let b: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    println!("dot prod a*b ({})", a.dot(b));
}
