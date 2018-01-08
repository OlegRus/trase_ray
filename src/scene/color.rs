use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, num: f32) -> Color {
        let r = (self.r as f32 * num) as u8;
        let g = (self.g as f32 * num) as u8;
        let b = (self.b as f32 * num) as u8;
        Color { r, g, b }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        let r = self.r + color.r;
        let g = self.g + color.g;
        let b = self.b + color.b;
        Color { r, g, b }
    }
}