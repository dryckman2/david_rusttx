use crate::hittables::hittable::HitRecord;
use crate::materials::material::Material;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use enum_dispatch::enum_dispatch;

pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropical;

use crate::math_structures::vec3::Point3;

#[derive(Clone)]
#[enum_dispatch]
pub enum MatEnum {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
    DiffuseLight(diffuse_light::DiffuseLight),
    Isotropic(isotropical::Isotropic),
}



