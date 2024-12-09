use std::sync::Arc;

use crate::{
    color::Color,
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian::new(Color::new())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = -(*outward_normal);
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
