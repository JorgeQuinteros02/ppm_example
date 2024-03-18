mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod interval;


use indicatif::ProgressBar;
use rtweekend::*;
use hittable::*;
use hittable_list::*;
use std::rc::Rc;
use crate::interval::*;
use crate::color::*;
use crate::ray::*;
use crate::vec3::*;
use crate::sphere::Sphere;

fn ray_color<T:Hittable>(r: &Ray, world: &T ) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
        return (rec.normal + Color::new(1.0,1.0,1.0)) * 0.5;
    }

    let unit_direction = r.direction() / (r.direction().norm2().sqrt());
    let a = (unit_direction.y + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0)*(1.0 - a) + Color::new(0.5, 0.7, 1.0)*a;
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;

    //Calculate image height and ensure it is at least 1.
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let bar = ProgressBar::new(image_height as u64 - 1);

    // World
    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new((0.0,0.0,-1.0),0.5)));
    world.add(Rc::new(Sphere::new((0.0,-100.5,-1.0),100.0)));


    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0; // Viewport widths less than one are ok since they are real valued.
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0,0.0,0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3"); // The colors are in ASCII
    println!("{image_width} {image_height}"); // specifying number of columns and rows
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            
            let pixel_color = ray_color(&r, &world);

            write_color(pixel_color)
        }
        bar.inc(1);
    }
}
