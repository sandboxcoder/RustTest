use std::ops;
use std::fmt;
pub struct Vector3 {
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
impl Vector3 {
    pub fn dot(&self, other : &Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn mag(&self) -> f32 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

    pub fn cross(&self, b : &Vector3) -> Vector3 {
        Vector3 { x: self.y*b.z - self.z*b.y,
            y: self.z*b.x - self.x*b.z,
            z: self.x*b.y - self.y*b.x,
        }
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

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f32 = 1e-6;
        (self.x - other.x).abs() < EPSILON &&
        (self.y - other.y).abs() < EPSILON &&
        (self.z - other.z).abs() < EPSILON
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write out the string in the format you want
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        // These vectors are orthogonal (perpendicular), so their dot product is zero.
        let a: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
        let b: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(a.dot(b), 0.0);

        let a: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
        let b: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
        assert_eq!(a.dot(b), 0.0);
    }

    #[test]
    fn test_add() {
        // These vectors are orthogonal (perpendicular), so their dot product is zero.
        let a: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
        let b: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        let c: Vector3 = a + b;
        let result: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 0.0 };
        assert!(c == result);
    }
}
