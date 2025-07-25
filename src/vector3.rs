
pub struct Vector3 {
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
impl Vector3 {
    pub fn dot(&self, other : Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z + other.z;
    }
}