use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::pdf::pdf::Pdf;

pub struct HittablePdf {
    objects: HittableList,
    origin: Point3,
}

impl HittablePdf {
    pub fn from(objects: HittableList, origin: Point3) -> HittablePdf {
        HittablePdf { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
