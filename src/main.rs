// Represents a simple RGB color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Color]>,
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
        format!("P3\n{} {}\n255\n", self.width, self.height)
            + &self
                .pixels
                .chunks(self.height)
                .into_iter()
                .map(|row| {
                    row.iter()
                        .map(|color| {
                            format!(
                                "{} {} {}\n",
                                (255.999 * color.red) as usize,
                                (255.999 * color.green) as usize,
                                (255.999 * color.blue) as usize
                            )
                        })
                        .reduce(|acc, e| acc + &e)
                        .unwrap()
                })
                .reduce(|acc, e| acc + &e)
                .unwrap()
    }
}

fn main() {
    let img = Image::new(256, 256);
    println!("{}", img.to_ppm());
}
