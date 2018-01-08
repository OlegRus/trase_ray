use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn null_vec() -> Vector {
        Vector::new(0., 0., 0.)
    }

    pub fn get_len(self) -> f32 {
        (self * self).sqrt()
    }

    pub fn normalized(self) -> Vector {
        self / self.get_len()
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector{ x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<Vector> for Vector {
    type Output = f32;

    fn mul(self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, num: f32) -> Vector {
        Vector { x: self.x * num, y: self.y * num, z: self.z * num }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, num: f32) -> Vector {
        Vector { x: self.x / num, y: self.y / num, z: self.z / num }
    }
}