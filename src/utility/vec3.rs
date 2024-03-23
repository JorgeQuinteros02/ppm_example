use vector3d::Vector3d;
use std::ops;

use crate::utility::rand;
pub type Vec3 = Vector3d<f64>;

pub trait Mul {
    fn mul(self, rhs: Self) -> Self;
}

impl<T:ops::Mul<Output = T  >> Mul for Vector3d<T> {
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Vector3d::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

#[inline]
pub fn random_vector() -> Vec3{
    Vec3::new(rand::random_double(), rand::random_double(), rand::random_double())
}

#[inline]
pub fn random_vector_range(min:f64, max:f64) -> Vec3 {
    let r = rand::random_double_range(min, max);
    let g = rand::random_double_range(min, max);
    let b = rand::random_double_range(min, max);
    Vec3::new(r, g, b)
}

#[inline]
pub fn unit_vector(vector: Vec3) -> Vec3 {
    vector / vector.norm2().sqrt()
}

#[inline]
fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        if p.norm2() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3{
    unit_vector(random_in_unit_sphere())
}

#[allow(unused)]
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline]
pub fn near_zero(v:Vec3) -> bool {
    //Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n*(v.dot(n)*2.0)
}

#[inline]
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = 1.0f64.min((-uv).dot(n));
    let r_out_perp = (uv + n*cos_theta)*etai_over_etat;
    let r_out_parallel = n * (-(((1.0 - r_out_perp.norm2()).abs()).sqrt()));
    
    r_out_perp + r_out_parallel
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random_double_range(-1.0,1.0), rand::random_double_range(-1.0,1.0), 0.0);
        if p.norm2() < 1.0 {
            return p
        }
    }
}

#[inline]
pub fn vec_from_tuple(tuple: (f64,f64,f64)) -> Vec3 {
    Vec3{x:tuple.0, y:tuple.1, z:tuple.2}
}