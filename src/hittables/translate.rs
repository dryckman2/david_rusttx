use crate::hittables::hittable::{HitRecord, Hittable};
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn from(p: Arc<dyn Hittable + Send + Sync>, displacement: Vec3) -> Translate {
        let mut this = Translate {
            object: p,
            offset: displacement,
            bbox: Aabb::blank(),
        };
        this.bbox = &this.object.bounding_box() + &this.offset;
        this
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::from_set_time(r.origin() - &self.offset, *r.direction(), r.time());
        // Determine where (if any) an intersection occurs along the offset ray
        let mut rec;
        match self.object.hit(&offset_r, ray_t) {
            None => {
                return None;
            }
            Some(x) => {
                rec = x;
            }
        }
        // Move the intersection point forwards by the offset
        rec.p += &self.offset;

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Send + Sync> {
        Box::new((*self).clone())
    }
}
