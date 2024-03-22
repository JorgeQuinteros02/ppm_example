use std::rc::Rc;

use crate::utility::{ray::Ray, vec3::Vec3};

use super::{aabb::Aabb, Hittable};

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb
}

impl Translate {
    pub fn new(p: Rc<dyn Hittable>, displacement:Vec3) -> Self{
        Translate{
            object:p.clone(),
            offset:displacement,
            bbox:p.bounding_box() + displacement
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::utility::ray::Ray, ray_t: crate::utility::interval::Interval, rec: &mut super::HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_timed(r.origin() - self.offset, r.direction(), r.time());

        // Determine where (if any) an intersection occurs along the offset ray
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false
        }

        //Move the intersection point forwards by the offset
        rec.p = rec.p + self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}