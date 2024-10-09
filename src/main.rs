mod image;
mod vector;
mod ray;
mod scene;
mod geometry;
mod util;
use vector::Vec3;

use crate::{geometry::{HittableList, Sphere}, scene::{Camera, Scene}};

fn main() {
    let world = HittableList::new()
        .add(Box::new(Sphere{ center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 }))
        .add(Box::new(Sphere{ center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 }));
    let camera = Camera{ position: Vec3::zero(), focal_length: 1.0, samples_per_pixel: 100, max_depth: 10 };
    let scene = Scene::new(16.0/9.0,  8.0, camera, world);
    let img = scene.render(800);
    println!("{}", img.to_ppm());
}
