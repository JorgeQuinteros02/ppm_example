// Uses
pub use crate::interval::*;
pub use crate::color::*;
pub use crate::vec3::*;
pub use crate::ray::*;
pub use std::rc::Rc;
use rand::Rng;

// Constants
pub const INFINITY:f64 = f64::INFINITY;
pub const PI:f64 = std::f64::consts::PI;


//Utility Functions
pub fn random_int_range(min:i32, max:i32) -> i32 {
    return rand::thread_rng().gen_range(min..max);
}


pub fn random_double() -> f64 {
    return rand::thread_rng().gen_range(0.0..1.0);
}

pub fn random_double_range(min:f64, max:f64) -> f64 {
    return min + (max-min)*random_double();
}