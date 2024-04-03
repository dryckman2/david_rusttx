use crate::math_structures::onb::Onb;
use crate::math_structures::vec3::Vec3;
use crate::pdf::pdf::Pdf;
use crate::rtweekend::PI;

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn from(w: &Vec3) -> CosinePdf {
        let mut this = CosinePdf { uvw: Onb::blank() };
        this.uvw.build_from_w(w);
        this
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = Vec3::dot(&Vec3::unit_vector(direction), self.uvw.w());
        f64::max(0.0, cosine_theta / PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_from_vec3(&Vec3::random_cosine_direction())
    }
}
