use std::ops::{self};

use crate::{
    point::Point3,
    ray::Ray,
    vec3::{Vec3, dot},
};

pub struct HitRecord {
    pub p: Point3,
    pub n: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, n: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            n,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_range: ops::Range<f64>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_range: ops::Range<f64>) -> Option<HitRecord> {
        let vcq = self.center - ray.origin();
        let a = ray.direction().len_sqrd();
        let h = dot(*ray.direction(), vcq);
        let c = vcq.len_sqrd() - (self.radius * self.radius);

        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let check_root = |root: f64| -> Option<HitRecord> {
            if ray_range.contains(&root) {
                let out_normal = (ray.at(root) - self.center) / self.radius;
                let front_face = dot(*ray.direction(), out_normal) < 0.0;
                let normal = if front_face { out_normal } else { -out_normal };

                Some(HitRecord::new(ray.at(root), normal, root, front_face))
            } else {
                None
            }
        };

        let sqrtd = discriminant.sqrt();

        check_root((h - sqrtd) / a).or_else(|| check_root((h + sqrtd) / a))
    }
}

pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, ray_range: ops::Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = ray_range.end;
        let mut temp_hit: Option<HitRecord> = None;

        for object in self.objects.iter() {
            let closest_range = ray_range.start..closest_so_far;

            if let Some(hit) = object.hit(ray, closest_range) {
                closest_so_far = hit.t;
                temp_hit = Some(hit);
            }
        }

        temp_hit
    }
}
