mod image;
mod vector;
mod ray;
mod scene;
mod geometry;
use vector::Vec3;

use crate::scene::Scene;

fn main() {
    let scene = Scene::new(16.0/9.0, 400, 8.0, Vec3::zero(), 1.0);
    let img = scene.render();
    println!("{}", img.to_ppm());
}
