use crate::utility::{color::Color, vec3::Vec3};

use super::Texture;

pub struct SolidColor {
    color_value:Color,
}

impl SolidColor {
    pub fn new(c:Color) -> Self{
        SolidColor{color_value:c}
    }

/*     pub fn from(r:f64, g:f64, b:f64) -> Self{
        SolidColor{color_value:Color::new(r, g, b)}
    } */
}

impl Texture for SolidColor {
     fn value(&self, _u:f64, _v:f64, _p:Vec3) -> Color {
         self.color_value
     }
}