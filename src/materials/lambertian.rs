use crate::hittables::hittable::HitRecord;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Vec3;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &rec.normal + &Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::from_set_time(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}
