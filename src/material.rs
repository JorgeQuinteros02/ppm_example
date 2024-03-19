use std::default;

use crate::{ray, rtweekend::*};
use crate::hittable::HitRecord;


pub trait Material {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;
}


pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Mat {
        Option::Some(Rc::new(Lambertian { albedo:*a}))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        // Catch degenerate scatter direction
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        
        return true;
    }
}

pub struct Metal {
    albedo:Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a:&Color, f:f64) -> Mat {
        let fuzz = if f < 1.0 {f} else {1.0};
        Option::Some(Rc::new(Metal { albedo:*a, fuzz }))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + random_unit_vector()*self.fuzz);
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}

pub type Mat = Option<Rc<dyn Material>>;

impl Material for Mat {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        match self {
            None => false,
            Some(T) => T.scatter(r_in, rec, attenuation, scattered)
        }
    }
}