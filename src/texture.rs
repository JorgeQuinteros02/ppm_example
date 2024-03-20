use crate::{rtw_image::RTWImage, rtweekend::*};

pub trait Texture {
    fn value(&self, u:f64, v:f64, p:Vec3) -> Color;
}

pub struct SolidColor {
    color_value:Color,
}

impl SolidColor {
    pub fn new(c:Color) -> Self{
        SolidColor{color_value:c}
    }

    pub fn from(r:f64, g:f64, b:f64) -> Self{
        SolidColor{color_value:Color::new(r, g, b)}
    }
}

impl Texture for SolidColor {
     fn value(&self, u:f64, v:f64, p:Vec3) -> Color {
         self.color_value
     }
}

pub struct Checkered {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl Checkered {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Checkered {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

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
        let xInteger = (self.inv_scale * p.x).floor() as i32;
        let yInteger = (self.inv_scale * p.y).floor() as i32;
        let zInteger = (self.inv_scale * p.z).floor() as i32;

        let isEven:bool = (xInteger + yInteger + zInteger) % 2 == 0;

        if isEven {self.even.value(u, v, p)} else {self.odd.value(u, v, p)}
    }
}

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
    fn value(&self, u:f64, v:f64, p:Vec3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {return Color::new(0.0, 1.1, 1.1)}

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