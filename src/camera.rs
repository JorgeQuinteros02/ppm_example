use std::default;

use crate::rtweekend::*;
use crate::hittable::*;
use indicatif::ProgressBar;

pub struct Camera {
    pub aspect_ratio:f64,
    pub image_width:i32,
    pub samples_per_pixel:i32,
    pub max_depth: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world:&impl Hittable) {
        self.initialize();

        println!("P3"); // The colors are in ASCII
        println!("{} {}", self.image_width, self.image_height); // specifying number of columns and rows
        println!("255");

        let bar = ProgressBar::new(self.image_height as u64 - 1);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i,j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world)
                }
                write_color(pixel_color, self.samples_per_pixel)
            }
            bar.inc(1);
        }
    }

    fn initialize(&mut self) {
        //Calculate image height and ensure it is at least 1.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0; // Viewport widths less than one are ok since they are real valued.
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Vec3::new(0.0,0.0,0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

    }

    fn ray_color (&self, r: &Ray, depth:i32, world:&impl Hittable) -> Color {
        let mut rec = HitRecord::default();
        
        if depth <= 0 {return Color::new(0.0, 0.0, 0.0)}

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = rec.normal + random_unit_vector();
            return self.ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5;
        }
    
        let unit_direction = r.direction() / (r.direction().norm2().sqrt());
        let a = (unit_direction.y + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0)*(1.0 - a) + Color::new(0.5, 0.7, 1.0)*a;
    }

    fn get_ray(&self, i: i32, j:i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
      
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio:1.0,
            image_width:100,
            samples_per_pixel:10,
            max_depth:10,
            image_height:0,
            center:Vec3::default(),
            pixel00_loc:Vec3::default(),
            pixel_delta_u:Vec3::default(),
            pixel_delta_v:Vec3::default(),
        }
    }
}