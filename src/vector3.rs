use std::ops;
pub struct Vector3 {
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
impl Vector3 {
    pub fn dot(&self, other : Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 { x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
