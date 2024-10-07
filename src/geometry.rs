use crate::{ray::Ray, util::Interval, vector::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere{
    pub center: Vec3,
    pub radius: f64,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(mut self, hittable: Box<dyn Hittable>) -> HittableList {
        self.objects.push(hittable);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { p, normal, t, front_face: false}
    }

    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: &Vec3) -> HitRecord {
        self.front_face = Vec3::dot(ray.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        self
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
       let oc = self.center - ray.origin();
       let a = ray.direction().length_squared();
       let h = Vec3::dot(ray.direction(), oc);
       let c = oc.length_squared() - self.radius * self.radius;

       let discriminant = h*h - a*c;
       if discriminant < 0.0 {
           return None;
       }

       let sqrtd = f64::sqrt(discriminant);
       let root_a = (h - sqrtd) / a;
       let root_b = (h + sqrtd) / a;

       if !ray_t.surrounds(root_a) && !ray_t.surrounds(root_b) {
           return None;
       }

       let outward_normal = (ray.at(root_a) - self.center) / self.radius;
       Some(HitRecord::new(root_a, ray.at(root_a), (ray.at(root_a) - self.center) / self.radius)
           .set_face_normal(ray, &outward_normal))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest = ray_t.max;
        let mut hit_record = None;

        for hittable in &self.objects {
            let hit = hittable.hit(ray, Interval::new(ray_t.min, closest));
            if hit.is_some() {
                closest = hit.unwrap().t;
                hit_record = hit;
            }
        }
        hit_record
    }
}
