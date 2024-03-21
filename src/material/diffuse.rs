use super::{Material, Mat, HitRecord, utility::{vec3, ray::Ray, color::Color}};
use crate::texture::{Texture, solid_color::SolidColor};
use std::rc::Rc;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: &Color) -> Mat {
        Rc::new(Lambertian {
            albedo:Rc::new(SolidColor::new(*a))
        })
    }

    pub fn from_texture(a: Rc<dyn Texture>) -> Mat {
        Rc::new(Lambertian{
            albedo: a
        })
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        
        // Catch degenerate scatter direction
        if vec3::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new_timed(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        
        return true;
    }
}
