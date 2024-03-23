
pub mod aabb;
pub mod bvh;
pub mod sphere;
pub mod hittable_list;
pub mod quad;
pub mod translate;
pub mod rotate_y;
pub mod constant_medium;

use crate:: utility::{vec3::Vec3, ray::Ray, interval::Interval};
use crate::material::Material;
use aabb::Aabb;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    mat: Option<Arc<dyn Material>>,
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

    pub fn mat(&self) -> Arc<dyn Material>{
        self.mat.clone().unwrap()
    }
}



pub trait Hittable: Sync + Send{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}