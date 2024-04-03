use crate::hittables::hittable::HitRecord;
use crate::materials::material::{Material, ScatterRecord};
use crate::math_structures::color::Color;
use crate::math_structures::onb::Onb;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
use crate::pdf::cosine_pdf::CosinePdf;
use crate::rtweekend::PI;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;
use crate::textures::TexEnum;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: TexEnum,
}

impl Lambertian {
    pub fn from_texture(a: TexEnum) -> Lambertian {
        Lambertian { albedo: a }
    }
    pub fn from_color(a: Color) -> Lambertian {
        Lambertian {
            albedo: TexEnum::SolidColor(SolidColor::from(a)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut srec = ScatterRecord::blank();
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Box::new(CosinePdf::from(&rec.normal));
        srec.skip_pdf = false;

        Some(srec)
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direction()));
        return if cos_theta < 0.0 { 0.0 } else { cos_theta / PI };
    }
}
