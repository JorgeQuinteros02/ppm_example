use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::interval::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: (f64,f64,f64), radius: f64) -> Sphere{
        Sphere{center:Vec3::new(center.0,center.1,center.2), radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t:Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().norm2();
        let half_b = oc.dot(r.direction());
        let c = oc.norm2() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;


        if discriminant < 0.0 {return false}
        
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}