pub mod perlin;
use super::Texture;
use perlin::Perlin;
use crate::utility::{vec3::Vec3, color::Color};

#[derive(Default)]
pub struct NoiseTexture {
    noise:Perlin,
    scale:f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise:Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u:f64, _v:f64, p:Vec3) -> crate::utility::color::Color {
        let s = p * self.scale;
        Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + (s.z + 10.0 *self.noise.turb(s, 7)).sin())
    }
}