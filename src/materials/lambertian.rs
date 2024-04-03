use crate::hittables::hittable::HitRecord;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::onb::Onb;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord,pdf:f64) -> Option<(Color, Ray,f64)> {
        let mut uvw = Onb::blank();
        uvw.build_from_w(&rec.normal);
        let scatter_direction = uvw.local_from_vec3(&Vec3::random_cosine_direction());

        let scattered = Ray::from_set_time(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = Vec3::dot(uvw.w(), scattered.direction()) / PI;

        Some((attenuation, scattered,pdf))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord)->f64{
        1.0/ (4.0 / PI)
    }
}
