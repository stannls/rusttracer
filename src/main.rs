mod image;
mod vector;
mod ray;
mod scene;
use image::Image;
use vector::Vec3;
use lazy_static::lazy_static;

use crate::scene::Scene;

fn main() {
    let scene = Scene::new(16.0/9.0, 400, 2.0, Vec3::zero(), 1.0);
    let img = scene.render();
    println!("{}", img.to_ppm());
}
