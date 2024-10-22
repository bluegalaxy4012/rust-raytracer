#[derive(Debug)] //momentan
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vector3 {
    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn normalize(&self) -> Self {
        let size = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if size > 0.0 {
            Vector3 {
                x: self.x / size,
                y: self.y / size,
                z: self.z / size,
            }
        } else {
            Vector3::zero()
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
