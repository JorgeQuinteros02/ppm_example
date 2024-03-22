use std::rc::Rc;

use crate::{material::{isotropic::Isotropic, Material}, texture::Texture, utility::{color::Color, interval::Interval, rand, vec3::Vec3}};

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    #[allow(unused)]
    pub fn new(b:Rc<dyn Hittable>, d:f64, a:Rc<dyn Texture>) -> Self {
        ConstantMedium{
            boundary:b,
            neg_inv_density:-1.0/d,
            phase_function:Rc::new(Isotropic::new(a)),
        }
    }

    pub fn from_color(b:Rc<dyn Hittable>, d:f64, c:Color) -> Self {
        ConstantMedium{
            boundary:b,
            neg_inv_density:-1.0/d,
            phase_function:Rc::new(Isotropic::from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::utility::ray::Ray, ray_t: crate::utility::interval::Interval, rec: &mut super::HitRecord) -> bool {
        // Print occasional samples when devugging. To enable, set enableDebug true.
        let enable_debug = false;
        let debugging = enable_debug && rand::random_double() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false
        }

        if !self.boundary.hit(r, Interval::new(rec1.t+0.0001, f64::INFINITY), &mut rec2) {
            return false
        }

        if debugging {eprintln!("\nray_tmin={}, ray_tmax={}\n",rec1.t,rec2.t);}

        if rec1.t < ray_t.min {rec1.t = ray_t.min;}
        if rec2.t > ray_t.max {rec2.t = ray_t.max;}

        if rec1.t >= rec2.t {
            return false
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().norm2().sqrt();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}\nrec.t = {}\nrec.p = {}",hit_distance, rec.t, rec.p);
        }

        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.mat = Option::Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> super::aabb::Aabb {
        self.boundary.bounding_box()
    }
}