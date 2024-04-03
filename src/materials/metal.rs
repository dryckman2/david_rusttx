use crate::hittables::hittable::HitRecord;
use crate::materials::material::{Material, ScatterRecord};
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::reflect;
use crate::math_structures::vec3::Vec3;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn from(a: Color, f: f64) -> Metal {
        let fuzz = f64::min(f, 1.0);
        Metal { albedo: a, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec = ScatterRecord::blank();
        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        srec.skip_pdf_ray = Ray::from_set_time(
            rec.p,
            &reflected + &(self.fuzz * &Vec3::random_in_unit_sphere()),
            r_in.time(),
        );
        Some(srec)
    }
}
