mod vector3;

use vector3::Vector3;

fn main() {
    let right: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    let up: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    println!("dot prod right*up ({})", right.dot(&up));
    println!("cross prod right x up ({})", right.cross(&up));
    println!("magnitude of vector right ({})", right.mag());
    let forward: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    println!("cross prod right x forward ({})", right.cross(&forward));
}
