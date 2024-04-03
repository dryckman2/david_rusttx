use crate::hittables::hittable::HitRecord;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord,pdf:f64) -> Option<(Color, Ray,f64)> {
        let scattered = Ray::from_set_time(rec.p, Vec3::random_unit_vector(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered,pdf))
    }
}
