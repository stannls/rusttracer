use std::ops;

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

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Color]>,
}

impl Color {
    pub fn to_ppm(&self) -> String {
        format!("{} {} {}\n", (255.999 * self.red) as usize, (255.999 * self.green) as usize, (255.999 * self.blue) as usize)
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
