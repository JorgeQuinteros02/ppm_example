use crate::utility::{ray::Ray, interval::Interval};
use std::rc::Rc;
use super::{Hittable, HitRecord, aabb::AABB};



#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        HittableList{
            objects: vec![object],
            bbox: AABB::default(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = AABB::from_boxes(self.bbox, object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t:Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec =  HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}