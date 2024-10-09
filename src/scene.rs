use crate::{geometry::{Hittable, HittableList}, image::{Color, Image}, ray::Ray, util::random_double, vector::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub focal_length: f64,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

pub struct Scene {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera: Camera,
    world: Box<dyn Hittable>,
}

impl Scene {
    pub fn new(aspect_ratio: f64,  viewport_height: f64, camera: Camera, world: HittableList) -> Scene {
        Scene { aspect_ratio, viewport_height, viewport_width: viewport_height * aspect_ratio, camera, world: Box::new(world)}
    }

    pub fn render(&self, image_width: usize) -> Image {
        let image_height = (image_width as f64 / self.aspect_ratio) as usize;
        let mut img = Image::new(image_width, image_height);

        let mut c = 0;
        for j in 0..image_height {
            for i in 0..image_width {
                let mut pixel_color = Color{ red: 0.0, green: 0.0, blue: 0.0 };
                for _sample in 0..self.camera.samples_per_pixel {
                    let r = self.get_ray(i, j, image_width, image_height);
                    pixel_color += r.color(self.world.as_ref(), self.camera.max_depth);
                }
                img.pixels[c] = pixel_color * (1.0 / self.camera.samples_per_pixel as f64);
                c+=1;
            }
        }

        img
    }

    fn get_ray(&self, i: usize, j: usize, image_width: usize, image_height: usize) -> Ray {
        // Camera
        let viewport_horizontal = Vec3::new(self.viewport_width, 0.0, 0.0);
        let viewport_vertical = Vec3::new(0.0, -self.viewport_height, 0.0);

        // Deltas
        let horizontal_delta = viewport_horizontal / image_width as f64;
        let vertical_delta = viewport_vertical / image_height as f64;

        // Viewport
        let viewport_upper_left = self.camera.position - Vec3::new(0.0, 0.0, self.camera.focal_length) - viewport_horizontal/2 - viewport_vertical/2;
        let pixel00_loc = viewport_upper_left + (horizontal_delta + vertical_delta) * 0.5;

        let offset = Scene::sample_square();
        let pixle_sample = pixel00_loc
            + (horizontal_delta * (i as f64 + offset.x()))
            + (vertical_delta * (j as f64 + offset.y()));
        
        let origin = self.camera.position;
        let direction = pixle_sample - origin;
        Ray::new(origin, direction)

    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}

