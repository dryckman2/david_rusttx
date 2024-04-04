use crate::math_structures::vec3::Vec3;
use crate::pdf::pdf::Pdf;
use crate::rtweekend::random_double;

pub struct MixturePdf {
    p: Vec<Box<dyn Pdf>>,
}

impl MixturePdf {
    pub fn from(p0: Box<dyn Pdf>, p1: Box<dyn Pdf>) -> MixturePdf {
        MixturePdf { p: vec![p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        return if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        };
    }
}
