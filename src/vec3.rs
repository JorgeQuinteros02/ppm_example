use vector3d::Vector3d;
use std::ops;

use crate::{random_double, random_double_range};

pub type Vec3 = Vector3d<f64>;

pub trait Mul {
    fn mul(self, rhs: Self) -> Self;
}

impl<T:ops::Mul<Output = T  >> Mul for Vector3d<T> {
    fn mul(self, rhs: Self) -> Self {
        let result = Vector3d::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
        return result

    }
}

fn random_vector() -> Vec3{
    Vec3::new(random_double(), random_double(), random_double())
}

fn random_vector_range(min:f64, max:f64) -> Vec3 {
    let r = random_double_range(min, max);
    let g = random_double_range(min, max);
    let b = random_double_range(min, max);
    Vec3::new(r, g, b)
}

pub fn unit_vector(vector: Vec3) -> Vec3 {
    return vector / vector.norm2().sqrt();
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        if p.norm2() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3{
    return unit_vector(random_in_unit_sphere());
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn near_zero(v:Vec3) -> bool {
    //Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    return v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n*(v.dot(*n)*2.0)
}