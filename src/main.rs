mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
mod material;


use rtweekend::*;
use hittable_list::*;
use crate::sphere::Sphere;
use camera::*;
use material::*;



fn main() {
  
    let mut world = HittableList::default();
    
    let material_ground = Lambertian::new(&Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(&Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(&Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Rc::new(Sphere::new((0.0,-100.5,-1.0),100.0, material_ground) ));
    world.add(Rc::new(Sphere::new((0.0,0.0,-1.0),0.5, material_center)));
    world.add(Rc::new(Sphere::new((-1.0,0.0,-1.0),0.5, material_left)));
    world.add(Rc::new(Sphere::new((1.0,0.0,-1.0),0.5, material_right)));
    
  
    let mut cam = Camera::default();
    
  
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    
  
    cam.render(&world);
    
}
