
pub mod aabb;
pub mod bvh;
pub mod sphere;
pub mod hittable_list;

use crate:: utility::{vec3::Vec3, ray::Ray, interval::Interval};
use crate::material::Mat;
use aabb::AABB;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    mat: Option<Mat>,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}


impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: The parameter 'outward normal' is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    pub fn mat(&self) -> Mat{
        self.mat.clone().unwrap()
    }
}



pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}