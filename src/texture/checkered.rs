use crate::utility::{color::Color, vec3::Vec3};
use super::{Texture, solid_color::SolidColor};
use std::rc::Rc;

pub struct Checkered {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl Checkered {
/*     pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Checkered {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    } */

    pub fn from_colors(scale:f64, c1:Color, c2:Color) -> Self {
        Checkered {
            inv_scale:scale,
            even:Rc::new(SolidColor::new(c1)),
            odd:Rc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for Checkered {
    fn value(&self, u:f64, v:f64, p:Vec3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;

        let is_even:bool = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {self.even.value(u, v, p)} else {self.odd.value(u, v, p)}
    }
}