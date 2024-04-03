use crate::math_structures::vec3::Vec3;
use crate::pdf::pdf::Pdf;
use crate::rtweekend::PI;

pub struct SpherePdf {}

impl SpherePdf {
    pub fn blank() -> SpherePdf {
        SpherePdf {}
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}
