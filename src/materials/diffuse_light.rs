use crate::hittables::hittable::HitRecord;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Point3;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;
use crate::textures::TexEnum;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: TexEnum,
}

impl DiffuseLight {
    pub fn from_texture(a: TexEnum) -> DiffuseLight {
        DiffuseLight { emit: a }
    }
    pub fn from_color(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: TexEnum::SolidColor(SolidColor::from(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
