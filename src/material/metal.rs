use super::{Material, Mat, HitRecord, utility::{vec3, ray::Ray, color::Color}};
use std::rc::Rc;

pub struct Metal {
    albedo:Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a:&Color, f:f64) -> Mat {
        let fuzz = if f < 1.0 {f} else {1.0};
        Rc::new(Metal { albedo:*a, fuzz })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new_timed(rec.p, reflected + vec3::random_unit_vector()*self.fuzz, r_in.time());
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}
