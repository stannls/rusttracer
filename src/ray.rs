use crate::{geometry::Hittable, image::Color, vector::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, hittable: &Box<dyn Hittable>) -> Color {
        let hit_record = hittable.hit(self, 0.0, f64::INFINITY);
        if hit_record.is_some() {
            let normal = hit_record.unwrap().normal;
            (Color { red: 1.0, green: 1.0 , blue: 1.0 } + Color{ red: normal[0], green: normal[1], blue: normal[2] }) * 0.5
        } else {
            let a = (self.direction().unit_vector().y() + 1.0) * 0.5;
            Color { red: 1.0, green: 1.0, blue: 1.0 } * (1.0 - a) + Color { red: 0.5, green: 0.7, blue: 1.0 } * a
        }
    }
}
