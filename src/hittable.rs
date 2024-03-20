use std::default;

use crate:: rtweekend::*;
use crate::material::*;
use crate::aabb::AABB;



#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Mat,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}


pub type HittableObject = Option<Rc<dyn Hittable>>;
impl Hittable for HittableObject {
    fn bounding_box(&self) -> AABB {
        match self {
            Some(t) => t.bounding_box(),
            None => AABB::default(),
        }
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            Some(t) => t.hit(r, ray_t, rec),
            None => false,
        }
    }
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
}



pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}