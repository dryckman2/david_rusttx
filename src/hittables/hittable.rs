use crate::materials::MatEnum;
use crate::math_structures::aabb::Aabb;
use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: MatEnum,
    pub(crate) front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -&(outward_normal.clone())
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;

    fn clone_dyn(&self) -> Box<dyn Hittable + Send + Sync>;

    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::from(1.0, 0.0, 0.0)
    }
}

impl Clone for Box<dyn Hittable + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
