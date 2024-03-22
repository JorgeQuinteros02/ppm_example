mod rtw_image;

use crate::utility::{vec3::Vec3,color::Color,interval::Interval};
use super::Texture;
use rtw_image::RTWImage;


pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self{
        ImageTexture {
            image: RTWImage::new(filename)
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u:f64, v:f64, _p:Vec3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() == 0 {return Color::new(0.0, 1.1, 1.1)}

        // Clamp input texture coordinates to [0,1] x [1,0]
        
        let i = Interval::new(0.0, 1.0);
        let u = i.clamp(u);
        let v = 1.0 - i.clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);
        
        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel.0 as f64,
            color_scale * pixel.1 as f64, 
            color_scale * pixel.2 as f64
        )
    }
}