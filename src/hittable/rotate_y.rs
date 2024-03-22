use std::rc::Rc;

use crate::utility::{interval::Interval, ray::Ray, vec3::Vec3};

use super::{aabb::Aabb, HitRecord, Hittable};

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox:Aabb
}

impl RotateY {
    pub fn new(p:Rc<dyn Hittable>, angle:f64) -> RotateY {
        let object = p;
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in [0.0, 1.0] {
            for j in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    let x = i*bbox.x.max + (1.0-i)*bbox.x.min;
                    let y = j*bbox.y.max + (1.0-j)*bbox.y.min;
                    let z = k*bbox.z.max + (1.0-k)*bbox.z.min;

                    let newx =  cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                    
                }
            }
        }

        RotateY {
            object, sin_theta, cos_theta,
            bbox:Aabb::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Change the ray from the world space to object space
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta*r.origin()[0] - self.sin_theta*r.origin()[2];
        origin[2] = self.sin_theta*r.origin()[0] + self.cos_theta*r.origin()[2];

        direction[0] = self.cos_theta*r.direction()[0] - self.sin_theta*r.direction()[2];
        direction[2] = self.sin_theta*r.direction()[0] + self.cos_theta*r.direction()[2];

        let rotated_r = Ray::new_timed(origin, direction, r.time());

        // Determine where (if any) intersection occurs in object space
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false
        }

        // Change the intersection point from object space to world space
        let mut p = rec.p;

        p[0] =  self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
        p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];

        // Change the normal from object space to world space
        let mut normal = rec.normal;

        normal[0] =  self.cos_theta*rec.normal[0] + self.sin_theta*rec.normal[2];
        normal[2] = -self.sin_theta*rec.normal[0] + self.cos_theta*rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

