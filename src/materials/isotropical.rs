use crate::hittables::hittable::HitRecord;
use crate::materials::material::{Material, ScatterRecord};
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
use crate::pdf::sphere_pdf::SpherePdf;
use crate::rtweekend::PI;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;
use crate::textures::TexEnum;

#[derive(Clone)]
pub struct Isotropic {
    albedo: TexEnum,
}

impl Isotropic {
    pub fn from_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: TexEnum::SolidColor(SolidColor::from(c)),
        }
    }

    pub fn from_texture(a: TexEnum) -> Isotropic {
        Isotropic { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec = ScatterRecord::blank();
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Box::new(SpherePdf::blank());
        srec.skip_pdf = false;
        Some(srec)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
