mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod interval;
mod camera;


use rtweekend::*;
use hittable_list::*;
use crate::sphere::Sphere;
use camera::*;



fn main() {
  
    let mut world = HittableList::default();
    
  
    world.add(Rc::new(Sphere::new((0.0,0.0,-1.0),0.5)));
    world.add(Rc::new(Sphere::new((0.0,-100.5,-1.0),100.0)));
    
  
    let mut cam = Camera::default();
    
  
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    
  
    cam.render(&world);
    
}
