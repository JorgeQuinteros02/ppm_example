use vector3d::Vector3d;
use std::ops;

use crate::utility;
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

pub fn random_vector() -> Vec3{
    Vec3::new(utility::random_double(), utility::random_double(), utility::random_double())
}

pub fn random_vector_range(min:f64, max:f64) -> Vec3 {
    let r = utility::random_double_range(min, max);
    let g = utility::random_double_range(min, max);
    let b = utility::random_double_range(min, max);
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

/* pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
} */

pub fn near_zero(v:Vec3) -> bool {
    //Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    return v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n*(v.dot(n)*2.0)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = 1.0f64.min((-uv).dot(n));
    let r_out_perp = (uv + n*cos_theta)*etai_over_etat;
    let r_out_parallel = n * (-(((1.0 - r_out_perp.norm2()).abs()).sqrt()));
    
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(utility::random_double_range(-1.0,1.0), utility::random_double_range(-1.0,1.0), 0.0);
        if p.norm2() < 1.0 {
            return p;
        }
    }
}

pub fn vec_from_tuple(tuple: (f64,f64,f64)) -> Vec3 {
    Vec3::new(tuple.0, tuple.1, tuple.2)
}