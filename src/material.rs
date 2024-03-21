pub mod diffuse;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;

use crate::utility::vec3::Vec3;
use crate::utility::{self, color::Color, ray::Ray}; // pass utility::self to children
use crate::hittable::HitRecord;
use std::rc::Rc;



pub trait Material {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;

    fn emitted(&self, _u:f64, _v:f64, _p:&Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub type Mat = Rc<dyn Material>;



