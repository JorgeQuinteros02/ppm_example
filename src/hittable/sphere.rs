use crate::utility::{vec3::{self, Vec3}, interval::Interval, ray::Ray, PI};
use super::{Hittable, HitRecord, aabb::AABB};
use crate::material::Mat;

pub struct Sphere {
    center1: Vec3,
    center_vec: Vec3,
    radius: f64,
    mat: Mat,
    is_moving: bool,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Mat) -> Self{
        let rvec = vec3::vec_from_tuple((radius,radius,radius));
        Sphere{
            center1:center,
            center_vec:vec3::vec_from_tuple((0.0,0.0,0.0)),
            radius,
            mat:mat.clone(),
            is_moving: false,
            bbox: AABB::from_points(center - rvec, center + rvec),
        }
    }

    pub fn new_movable(center1: Vec3, center2: Vec3, radius: f64, mat: &Mat) -> Sphere{
        let rvec = vec3::vec_from_tuple((radius,radius,radius));
        let box1 = AABB::from_points(center1 - rvec, center1 + rvec);
        let box2 = AABB::from_points(center2 - rvec, center2 + rvec);
        


        
        Sphere{
            center1:center1,
            center_vec: center2 - center1,
            radius,
            mat:mat.clone(),
            is_moving: true,
            bbox: AABB::from_boxes(box1, box2),
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        // Linearly interpolate from center1 to center2 according to time, where t=0 yields
        // center1, and t=1 yields center2.
        self.center1 + self.center_vec * time
    }

    pub fn get_sphere_uv(p:Vec3, u:&mut f64, v:&mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        *u = phi / (2.0*PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t:Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {self.center(r.time())} else {self.center1};
        let oc = r.origin() - center;
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
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = Option::Some(self.mat.clone());

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}