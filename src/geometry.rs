use crate::{ray::Ray, vector::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere{
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn collides(&self, r: Ray) -> bool {
        let oc = self.center - r.origin();
        let a = Vec3::dot(r.direction(), r.direction());
        let b = -2.0 * Vec3::dot(r.direction(), oc);
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant >= 0.0
    }
}
