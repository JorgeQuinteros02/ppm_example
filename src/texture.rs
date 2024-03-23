use crate::utility::{vec3::Vec3, color::Color};

pub mod solid_color;
pub mod checkered;
pub mod image;
pub mod noise;
pub trait Texture: Sync + Send {
    fn value(&self, u:f64, v:f64, p:Vec3) -> Color;
}





