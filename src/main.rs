mod image;
mod vector;
use image::Image;

fn main() {
    let mut img = Image::new(256, 256);
    for j in 0..img.height {
        for i in 0..img.width {
            let pixel = &mut img.pixels[j*img.height + i];
            pixel.red = i as f32 / (img.width - 1) as f32;
            pixel.green = j as f32 / (img.height - 1) as f32;
            pixel.blue = 0.0;
        }
    }
    println!("{}", img.to_ppm());
}
