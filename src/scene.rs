use crate::{geometry::Sphere, image::{Color, Image}, ray::Ray, vector::Vec3};

pub struct Scene {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    viewport_height: f64,
    viewport_width: f64,
    camera_position: Vec3,
    focal_length: f64
}

impl Scene {
    pub fn new(aspect_ratio: f64, image_width: usize, viewport_height: f64, camera_position: Vec3, focal_length: f64) -> Scene {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        Scene { aspect_ratio, image_width, image_height, viewport_height, viewport_width: viewport_height * (image_width as f64 / image_height as f64), camera_position, focal_length }
    }

    pub fn render(&self) -> Image {
        let mut img = Image::new(self.image_width, self.image_height);

        // Camera
        let viewport_horizontal = Vec3::new(self.viewport_width, 0.0, 0.0);
        let viewport_vertical = Vec3::new(0.0, -self.viewport_height, 0.0);

        let horizontal_delta = viewport_horizontal / self.image_width as f64;
        let vertical_delta = viewport_vertical / self.image_height as f64;

        let viewport_upper_left = self.camera_position - Vec3::new(0.0, 0.0, self.focal_length) - viewport_horizontal/2 - viewport_vertical/2;
        let pixel00_loc = viewport_upper_left + (horizontal_delta + vertical_delta) * 0.5;

        let sphere = Sphere{ center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 };

        let mut c = 0;
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = pixel00_loc + (horizontal_delta * i as f64) + (vertical_delta * j as f64);
                let ray_direction = pixel_center - self.camera_position;
                let ray = Ray::new(self.camera_position, ray_direction);

                let color = sphere_color(ray, sphere);
                img.pixels[c] = color;
                c+=1;
            }
        }

        img
    }
}

fn ray_color(ray: Ray) -> Color {
    let a = 0.5*(ray.direction().y() + 1.0);
    Color { red: 1.0, green: 1.0, blue: 1.0 } * (1.0 - a) + Color { red: 0.5, green: 0.7, blue: 1.0 } * a
}

fn sphere_color(ray: Ray, sphere: Sphere) -> Color {
    if sphere.collides(ray) {
        Color{ red: 1.0, green: 0.0, blue: 0.0 }
    } else {
        let a = 0.5*(ray.direction().unit_vector().y() + 1.0);
        Color { red: 1.0, green: 1.0, blue: 1.0 } * (1.0 - a) + Color { red: 0.5, green: 0.7, blue: 1.0 } * a
    }
}
