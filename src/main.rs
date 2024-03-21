mod hittable;
mod utility;
mod camera;
mod material;
mod texture;


use utility::{rand, color::Color, vec3::{self, Vec3, Mul}};
use hittable::{
    bvh::BVHNode, hittable_list::HittableList, quad::{Quad, _box}, rotate_y::RotateY, sphere::Sphere, translate::Translate, Hittable
};
use material::{
    dielectric::Dielectric, diffuse::Lambertian, diffuse_light::DiffuseLight, metal::Metal
};
use texture::{
    checkered::Checkered,
    image::ImageTexture,
    noise::NoiseTexture
};
use camera::Camera;
use std::rc::Rc;
use std::env;




fn main() {
    let args: Vec<String> = env::args().collect();
    let choice = &args[1] as &str;
    match choice {
        "1" => random_spheres(),
        "2" => two_spheres(),
        "3" => earth(),
        "4" => two_perlin_spheres(),
        "5" => quads(),
        "6" => simple_light(),
        "7" => cornell_box(),
        _ => println!("Unrecognized option")
    };
}

fn random_spheres() {
      
    let mut world = HittableList::default();

    let checker = Rc::new(Checkered::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    world.add(
        Rc::new(
            Sphere::new(
                vec3::vec_from_tuple((0.0,-1000.0,0.0)), 
                1000.0, 
                Lambertian::from_texture(checker)
            )
        )
    );

    //let ground_material = Lambertian::new(&Color::new(0.5,0.5,0.5));
    //world.add(Sphere::new(vec_from_tuple((0.0,-1000.0,0.0)), 1000.0, &ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random_double();
            let center = Vec3::new(a as f64 + 0.9*rand::random_double(), 0.2, b as f64 + 0.9*rand::random_double());

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm2() > 0.9 {
                let sphere_material ;

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo: Color = vec3::random_vector().mul(vec3::random_vector());
                    sphere_material = Lambertian::new(&albedo);
                    let center2 = center + vec3::vec_from_tuple((0.0,rand::random_double_range(0.0, 0.5),0.0));
                    world.add(Rc::new(Sphere::new_movable(center, center2,0.2, &sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = vec3::random_vector_range(0.5,1.0);
                    let fuzz = rand::random_double_range(0.0, 0.5);
                    sphere_material = Metal::new(&albedo, fuzz);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Dielectric::new(1.5);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Rc::new(Sphere::new(vec3::vec_from_tuple((0.0,1.,0.0)), 1.0, material1)));

    let material2 = Lambertian::new(&Vec3::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::new(vec3::vec_from_tuple((-4.0,1.0,0.0)), 1.0, material2)));

    let material3 = Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Rc::new(Sphere::new(vec3::vec_from_tuple((4.0,1.0,0.0)), 1.0, material3)));

    let world = HittableList::new(Rc::new(BVHNode::new(&world)));

    let mut cam = Camera::default();
    
  
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    
  
    cam.render(&world);
}

fn two_spheres() {
    let mut world = HittableList::default();

    let checker = Rc::new(Checkered::from_colors(0.8, Color::new(0.2, 0.3,0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Rc::new(Sphere::new(vec3::vec_from_tuple((0.0,-10.0,0.0)), 10.0, Lambertian::from_texture(checker.clone()))));
    world.add(Rc::new(Sphere::new(vec3::vec_from_tuple((0.0, 10.0,0.0)), 10.0, Lambertian::from_texture(checker))));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);


    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Lambertian::from_texture(earth_texture);
    let globe = Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface
    ));

    let mut world = HittableList::default();
    world.add(globe);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);


    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 12.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);

}

fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    world.add(
        Rc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::from_texture(pertext.clone()))
        )
    );
    world.add(
        Rc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::from_texture(pertext))
        )
    );

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);


    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();

    // Materials
    let left_red        = Lambertian::new(&Color::new(1.0, 0.2, 0.2));
    let back_green      = Lambertian::new(&Color::new(0.2, 1.0, 0.2));
    let right_blue      = Lambertian::new(&Color::new(0.2, 0.2, 1.0));
    let upper_orange    = Lambertian::new(&Color::new(1.0, 0.5, 0.0));
    let lower_teal      = Lambertian::new(&Color::new(0.2, 0.8, 0.8));

    // Quads
    world.add(Rc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0), 
        Vec3::new(0.0,0.0,-4.0), 
        Vec3::new(0.0,4.0,0.0), 
        left_red))
    );
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0), 
        Vec3::new(4.0,0.0,0.0), 
        Vec3::new(0.0,4.0,0.0), 
        back_green))
    );
    world.add(Rc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0), 
        Vec3::new(0.0,0.0,4.0), 
        Vec3::new(0.0,4.0,0.0),
        right_blue))
    );
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0), 
        Vec3::new(4.0,0.0,0.0), 
        Vec3::new(0.0,0.0,4.0),
        upper_orange))
    );
    world.add(Rc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0), 
        Vec3::new(4.0,0.0,0.0), 
        Vec3::new(0.0,0.0,-4.0),
        lower_teal))
    );
    
    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);


    cam.vfov = 80.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
    

}

fn simple_light () {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
    world.add(
        Rc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::from_texture(pertext.clone()))
        )
    );
    world.add(
        Rc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::from_texture(pertext))
        )
    );

    let difflight = Rc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(
        Rc::new(Sphere::new(
            Vec3::new(0.0, 7.0, 0.0), 
            2.0, 
        difflight.clone()
        ))
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(3.0, 1.0, -2.0), 
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            difflight
        ))
    );

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);


    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    cam.lookat = Vec3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);

}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Lambertian::new(&Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(&Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(&Color::new(0.12, 0.45, 0.15));
    let light = Rc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(
        Rc::new(Quad::new(
            Vec3::new(555.0, 0.0, 0.0), 
            Vec3::new(0.0, 555.0, 0.0), 
            Vec3::new(0.0, 0.0, 555.0), 
        green)
        )
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 555.0, 0.0), 
            Vec3::new(0.0, 0.0, 555.0), 
        red)
        )
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(343.0, 554.0, 332.0), 
            Vec3::new(-130.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -105.0), 
        light)
        )
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(555.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, 555.0), 
        white.clone())
        )
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(555.0, 555.0, 555.0), 
            Vec3::new(-555.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -555.0), 
        white.clone())
        )
    );
    world.add(
        Rc::new(Quad::new(
            Vec3::new(0.0, 0.0, 555.0), 
            Vec3::new(555.0, 0.0, 0.0), 
            Vec3::new(0.0, 555.0, 0.0), 
        white.clone())
        )
    );

    let mut box1:Rc<dyn Hittable> = _box(
        Vec3::default(),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone()
    );

    box1 = Rc::new(RotateY::new(box1, -15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = _box(
        Vec3::default(),
        Vec3::new(165.0, 165.0, 165.0),
        white
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);
    
    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);


    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);

}