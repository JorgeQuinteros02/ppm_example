use image::io::Reader as ImageReader;
use image::{ImageBuffer, Rgb};

use crate::{Color, Interval};

#[derive(Default)]
pub struct RTWImage {
    data: Option< Vec<u8>>,
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl RTWImage {
    pub fn new(image_filename:&str) -> Self {
        let mut r = RTWImage::default();
        if r.load(image_filename) {r} else {panic!("Could not read {image_filename} into file")}
    }

    pub fn load(&mut self, image_filename:&str) -> bool {
        let img = match ImageReader::open(image_filename) {
            Ok(t) => match t.decode() {
                Ok(t) => t,
                Err(t) => return false
            },
            Err(t) => return false
        };

        self.image_height = img.height() as usize;
        self.image_width = img.width() as usize;
        self.bytes_per_scanline = self.image_width * 3;
        self.data = Option::Some((img.into_rgb8().into_vec())); // This consumes the img so we put it last
        
        
        true
    }

    pub fn height(&self) -> usize {
        self.image_height
    }

    pub fn width(&self) -> usize {
        self.image_width
    }

    pub fn pixel_data(&self, x: usize, y:usize) -> (u8,u8,u8) {
        match &self.data {
            None => (255,0,255),
            Some(v)=> {
                let x = RTWImage::clamp(x, 0 ,self.image_width);
                let y = RTWImage::clamp(y, 0, self.image_height);
                let i = y*self.bytes_per_scanline + x*3; // index of pixel in vector
                (v[i], v[i+1], v[i+2])
            }
        }
    }

    fn clamp(x:usize, low:usize, high:usize) -> usize {
        //Return the value clamped to the range [low, high)
        if x < low {low}
        else if x < high {x}
        else {high - 1}
    }
}
