
use rand::{thread_rng, Rng};

pub fn random_int_range(min:i32, max:i32) -> i32 {
    thread_rng().gen_range(min..max)
}


pub fn random_double() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min:f64, max:f64) -> f64 {
    min + (max-min)*random_double()
}