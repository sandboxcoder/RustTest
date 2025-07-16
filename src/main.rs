mod vector;

use vector::Vector;

fn main() {
    let origin: Vector = Vector { x: 5.2, y: 0.4, z: 6.0 };
    println!("origin ({} {} {})", origin.x, origin.y, origin.z);
}
