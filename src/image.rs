use std::ops;

use crate::util::Interval;

// Represents a simple RGB color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color{ red: self.red * rhs, green: self.green * rhs, blue: self.blue * rhs }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color{ red: self.red + rhs.red, green: self.green + rhs.green, blue: self.blue + rhs.blue }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red + rhs.red;
        self.green = self.green + rhs.green;
        self.blue = self.blue + rhs.blue;
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Color]>,
}

impl Color {
    pub fn to_ppm(&self) -> String {
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * intensity.clamp(self.red)) as usize;
        let gbyte = (256.0 * intensity.clamp(self.green)) as usize;
        let bbyte = (256.0 * intensity.clamp(self.blue)) as usize;

        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }
}

// Represents an Image consisting of multiple pixels
impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let pixels = vec![
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0
            };
            width * height
        ];
        Image {
            width,
            height,
            pixels: pixels.into_boxed_slice(),
        }
    }

    // Converts the pixels to ppm format
    pub fn to_ppm(&self) -> String {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height);
        for i in 0..self.pixels.len() {
            out += &self.pixels[i].to_ppm();
        }
        out
    }
}
