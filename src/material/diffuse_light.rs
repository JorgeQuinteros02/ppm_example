use std::rc::Rc;

use crate::{hittable::HitRecord, texture::{solid_color::SolidColor, Texture}, utility::{color::Color, ray::Ray, vec3::Vec3}};

use super::Material;

pub struct DiffuseLight {
    emit:Rc<dyn Texture>
}

impl DiffuseLight {
    #[allow(unused)]
    pub fn new(a:Rc<dyn Texture>) -> DiffuseLight{
        DiffuseLight {emit:a}
    }

    pub fn from_color(c: Color) -> DiffuseLight {
        DiffuseLight{emit:Rc::new(SolidColor::new(c))}
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u:f64, v:f64, p:Vec3) -> Color {
        self.emit.value(u, v, p)
    }

    fn scatter(&self, _r_in:&Ray, _rec:&HitRecord, mut _attenuation: &mut Color, _scattered:&mut Ray) -> bool {
        false
    }
}