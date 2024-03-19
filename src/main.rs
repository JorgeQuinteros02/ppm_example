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
    
    let ground_material = Lambertian::new(&Color::new(0.5,0.5,0.5));
    world.add(Rc::new(Sphere::new((0.0,-1000.0,0.0), 1000.0, &ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9*random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm2() > 0.9 {
                let center_tuple = (center.x, center.y, center.z);

                let mut sphere_material = Mat::default();

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo: Color = random_vector().mul(random_vector());
                    sphere_material = Lambertian::new(&albedo);
                    world.add(Rc::new(Sphere::new(center_tuple, 0.2, &sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = random_vector_range(0.5,1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(&albedo, fuzz);
                    world.add(Rc::new(Sphere::new(center_tuple, 0.2, &sphere_material)));
                } else {
                    // glass
                    sphere_material = Dielectric::new(1.5);
                    world.add(Rc::new(Sphere::new(center_tuple, 0.2, &sphere_material)));
                
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Rc::new(Sphere::new((0.0,1.,0.0), 1.0, &material1)));

    let material2 = Lambertian::new(&Vec3::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::new((-4.0,1.0,0.0), 1.0, &material2)));

    let material3 = Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Rc::new(Sphere::new((4.0,1.0,0.0), 1.0, &material3)));

    let mut cam = Camera::default();
    
  
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    
  
    cam.render(&world);
    
}
