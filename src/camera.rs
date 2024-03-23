use crate::utility::{
    rand,
    vec3::{self, Vec3, Mul},
    ray::Ray,
    color::{self, Color},
    interval::Interval,
    INFINITY
};
use crate::hittable::*;
use indicatif::ProgressBar;
use rayon::prelude::*;

pub struct Camera {
    pub aspect_ratio:f64,
    pub image_width:i32,
    pub samples_per_pixel:i32,
    pub max_depth: i32,
    pub background: Color,
    pub vfov: f64, // Vertical view angle
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world:&(impl Hittable + Sync)) {
        self.initialize();

        println!("P3"); // The colors are in ASCII
        println!("{} {}", self.image_width, self.image_height); // specifying number of columns and rows
        println!("255");

        let pixel_number = (self.image_height * self.image_width);

        let bar = ProgressBar::new((pixel_number) as u64);
        let mut pixels: Vec<Color> = vec![Color::default()];

        (0..pixel_number).into_par_iter().map(|n| -> Color{
            let j = n / self.image_width;
            let i = n - j * self.image_width;

            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _sample in 0..self.samples_per_pixel {
                let r = self.get_ray(i,j);
                pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
            }

            bar.inc(1);
            pixel_color
        }).collect_into_vec(&mut pixels);

        let write_bar = ProgressBar::new(pixel_number as u64);
        for c in pixels {
            color::write_color(c, self.samples_per_pixel);
            bar.inc(1);
        }

/*         for n in 0..(self.image_height * self.image_width) {

            let j = n / self.image_width;
            let i = n - j * self.image_width;
            
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _sample in 0..self.samples_per_pixel {
                let r = self.get_ray(i,j);
                pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
            }
            
            color::write_color(pixel_color, self.samples_per_pixel);


            bar.inc(1);
        }
 */    }

    fn initialize(&mut self) {
        //Calculate image height and ensure it is at least 1.
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist; // Viewport widths less than one are ok since they are real valued.
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(self.vup.cross(self.w));
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width;
        let viewport_v = (-self.v) * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disck basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }


    fn ray_color (&self, r: &Ray, depth:i32, world:&(impl Hittable + Sync)) -> Color {
        let mut rec = HitRecord::default();
        
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {return Color::new(0.0, 0.0, 0.0)}

        // If the ray hits nothing, return the background color.
        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background
        }
        
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emission = rec.mat().emitted(rec.u, rec.v, rec.p);

        if !rec.mat().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission
        }
        let color_from_scatter = attenuation.mul(self.ray_color(&scattered, depth-1, world));
        
        color_from_emission + color_from_scatter
        /* let unit_direction = r.direction() / (r.direction().norm2().sqrt());
        let a = (unit_direction.y + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0)*(1.0 - a) + Color::new(0.5, 0.7, 1.0)*a; */
    }

    fn get_ray(&self, i: i32, j:i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j originnating from
        // the camera defocus disk.
      
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        
        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rand::random_double();

        Ray::new_timed(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rand::random_double();
        let py = -0.5 + rand::random_double();

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
            background:Color::default(),
            vfov:90.0,
            lookfrom: Vec3::new(0.0, 0.0, -1.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height:0,
            center:Vec3::default(),
            pixel00_loc:Vec3::default(),
            pixel_delta_u:Vec3::default(),
            pixel_delta_v:Vec3::default(),
            u:Vec3::default(),
            v:Vec3::default(),
            w:Vec3::default(),
            defocus_disk_u:Vec3::default(),
            defocus_disk_v:Vec3::default(),
        }
    }
}