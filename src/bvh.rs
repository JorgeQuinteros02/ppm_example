use std::cmp::Ordering;

use crate::{aabb::AABB, hittable::{Hittable, HittableObject}, rtweekend::*, HittableList};

pub struct BVHNode {
    left: HittableObject,
    right: HittableObject,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut crate::hittable::HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);

        return hit_left || hit_right
    }
}

impl BVHNode {
    pub fn new(list: &HittableList) -> HittableObject {
        BVHNode::from_slice(&list.objects, 0, list.objects.len())
    }

    pub fn from_slice(src_objects: &Vec<HittableObject>, start: usize, end: usize) -> HittableObject {
        let mut objects = src_objects.clone();
        let mut left = HittableObject::default();
        let mut right = HittableObject::default();


        let axis = random_int_range(0,2) as usize;
        let comparator = |a:&HittableObject,b:&HittableObject|{BVHNode::box_compare(a.clone(),b.clone(),axis)};

        let object_span = end - start;

        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            },
            2 => {
                match comparator(&objects[start], &objects[start+1]) {
                    Ordering::Less => {
                        left = objects[start].clone();
                        right = objects[start+1].clone();
                    }
                    _other => {
                        left = objects[start+1].clone();
                        right = objects[start].clone();
                    }
                }
            },
            _other => {
                objects.sort_by(comparator);

                let mid = start + object_span / 2;
                left = BVHNode::from_slice(&objects, start, mid);
                right = BVHNode::from_slice(&objects, mid, end);
                
            }
        }

        

        Option::Some(Rc::new(BVHNode{
            left:left.clone(),
            right:right.clone(),
            bbox:AABB::from_boxes(left.bounding_box(), right.bounding_box())}))
    }

    pub fn box_compare(a:HittableObject, b:HittableObject, axis_index:usize) -> Ordering{
        a.bounding_box().axis(axis_index).min.partial_cmp(&b.bounding_box().axis(axis_index).min).unwrap()
    }
}