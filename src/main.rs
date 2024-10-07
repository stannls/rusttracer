mod image;
mod vector;
mod ray;
mod scene;
mod geometry;
mod util;
use vector::Vec3;

use crate::{geometry::{HittableList, Sphere}, scene::Scene};

fn main() {
    let world = HittableList::new()
        .add(Box::new(Sphere{ center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 }))
        .add(Box::new(Sphere{ center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 }));
    let scene = Scene::new(16.0/9.0, 800, 8.0, Vec3::zero(), 1.0, world, 100);
    let img = scene.render();
    println!("{}", img.to_ppm());
}
