pub mod diffuse;
pub mod metal;
pub mod dielectric;

use crate::utility::{self, color::Color, ray::Ray}; // pass utility::self to children
use crate::hittable::HitRecord;
use std::rc::Rc;


pub trait Material {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;
}

pub type Mat = Option<Rc<dyn Material>>;

impl Material for Mat {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        match self {
            None => {
                //println!("FALSE IS BEING CALLED");
                false
            },
            Some(t) => t.scatter(r_in, rec, attenuation, scattered)
        }
    }
}


