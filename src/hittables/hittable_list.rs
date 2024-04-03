use crate::hittables::hittable::{HitRecord, Hittable};
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::random_int_bounded;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    bbox: Aabb,
}

impl HittableList {
    pub(crate) fn blank() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb::blank(),
        }
    }

    pub(crate) fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
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

    fn clone_dyn(&self) -> Box<dyn Hittable + Send + Sync> {
        Box::from((*self).clone())
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        return sum;
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        if self.objects.len() == 0 {
            return Vec3::blank();
        }
        let int_size = self.objects.len() as i64;
        return self.objects[random_int_bounded(0, int_size - 1) as usize].random(o);
    }
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        let mut this = HittableList::blank();
        for x in &self.objects {
            this.add(x.clone());
        }
        this.bbox = self.bbox.clone();
        this
    }
}
