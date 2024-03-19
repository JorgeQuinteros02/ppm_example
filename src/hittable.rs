use std::default;

use crate:: rtweekend::*;
use crate::material::*;



#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Mat,
    pub t: f64,
    pub front_face: bool,
}

pub type HittableObject = Rc<dyn Hittable>;


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
}