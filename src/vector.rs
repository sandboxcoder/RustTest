
pub struct Vector {
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
impl Vector {
    pub fn dot(&self, other : Vector) -> f32 {
        return self.x * other.x + self.y * other.y + self.z + other.z;
    }
}