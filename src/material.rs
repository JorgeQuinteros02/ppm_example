use std::default;

use crate::texture::{SolidColor, Texture};
use crate::{ray, rtweekend::*};
use crate::hittable::HitRecord;


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

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: &Color) -> Mat {
        Option::Some(Rc::new(Lambertian {
            albedo:Rc::new(SolidColor::new(*a))
        }))
    }

    pub fn from_texture(a: Rc<dyn Texture>) -> Mat {
        Option::Some(Rc::new(Lambertian{
            albedo: a
        }))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        // Catch degenerate scatter direction
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new_timed(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        
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
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new_timed(rec.p, reflected + random_unit_vector()*self.fuzz, r_in.time());
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Mat {
        Option::Some(Rc::new(Dielectric { ir:index_of_refraction}))
    }

    pub fn reflectance(&self, cosine:f64, ref_idx:f64) -> f64 {
        //Use Schlick's approximation for reflectance.
        let mut  r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0)*((1.0-cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        *attenuation =  Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0 / self.ir} else {self.ir};

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::default();

        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, refraction_ratio);
        }
        
        *scattered = Ray::new_timed(rec.p, direction, r_in.time());
        true
    }
}