use crate::hittable::Hittable;
use std::rc::Rc;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::interval::*;


type HittableObject = Rc<dyn Hittable>;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<HittableObject>
}

impl HittableList {
    pub fn new(object: HittableObject) -> HittableList {
        return HittableList{objects: vec![object]};
    }

    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
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
}