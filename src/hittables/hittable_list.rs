use crate::math_structures::aabb::Aabb;
use crate::hittables::hittable::{HitRecord, Hittable};
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;

#[derive(Clone)]
pub struct HittableList {
    pub(crate) objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub(crate) fn blank() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb::blank(),
        }
    }

    pub(crate) fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = Aabb::from_aabbs(&self.bbox, &object.bounding_box());
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            match { object.hit(r, &Interval::from(ray_t.min, closest_so_far)) } {
                Some(x) => {
                    closest_so_far = x.t;
                    hit_anything = Some(x);
                }
                _ => {}
            };
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::from((*self).clone())
    }
}
