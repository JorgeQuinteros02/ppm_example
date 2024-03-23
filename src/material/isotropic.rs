use std::sync::Arc;

use crate::{texture::{solid_color::SolidColor, Texture}, utility::{color::Color, ray::Ray, vec3}};

use super::Material;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    #[allow(unused)]
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic{albedo}
    }

    pub fn from_color(c: Color) -> Self{
        Isotropic{albedo:Arc::new(SolidColor::new(c))}
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in:&crate::utility::ray::Ray, rec:&crate::hittable::HitRecord, attenuation: &mut Color, scattered:&mut Ray) -> bool {
        *scattered = Ray::new_timed(rec.p, vec3::random_unit_vector(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}