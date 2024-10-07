use crate::{geometry::{Hittable, HittableList, Sphere}, image::{Color, Image}, ray::Ray, vector::Vec3};
use rand::prelude::*;

pub struct Scene {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    viewport_height: f64,
    viewport_width: f64,
    camera_position: Vec3,
    focal_length: f64,
    world: Box<dyn Hittable>,
    samples_per_pixel: usize,
}

impl Scene {
    pub fn new(aspect_ratio: f64, image_width: usize, viewport_height: f64, camera_position: Vec3, focal_length: f64, world: HittableList, samples_per_pixel: usize) -> Scene {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        Scene { aspect_ratio, image_width, image_height, viewport_height, viewport_width: viewport_height * (image_width as f64 / image_height as f64), camera_position, focal_length, world: Box::new(world), samples_per_pixel }
    }

    pub fn render(&self) -> Image {
        let mut img = Image::new(self.image_width, self.image_height);

        let mut c = 0;
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color{ red: 0.0, green: 0.0, blue: 0.0 };
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += r.color(&self.world);
                }
                img.pixels[c] = pixel_color * (1.0 / self.samples_per_pixel as f64);
                c+=1;
            }
        }

        img
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Camera
        let viewport_horizontal = Vec3::new(self.viewport_width, 0.0, 0.0);
        let viewport_vertical = Vec3::new(0.0, -self.viewport_height, 0.0);

        // Deltas
        let horizontal_delta = viewport_horizontal / self.image_width as f64;
        let vertical_delta = viewport_vertical / self.image_height as f64;

        // Viewport
        let viewport_upper_left = self.camera_position - Vec3::new(0.0, 0.0, self.focal_length) - viewport_horizontal/2 - viewport_vertical/2;
        let pixel00_loc = viewport_upper_left + (horizontal_delta + vertical_delta) * 0.5;

        let offset = Scene::sample_square();
        let pixle_sample = pixel00_loc
            + (horizontal_delta * (i as f64 + offset.x()))
            + (vertical_delta * (j as f64 + offset.y()));
        
        let origin = self.camera_position;
        let direction = pixle_sample - origin;
        Ray::new(origin, direction)

    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(0.0..1.0) - 0.5, rng.gen_range(0.0..1.0) - 0.5, 0.0)
    }
}

