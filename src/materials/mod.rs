//noinspection RsUnusedImport
use crate::hittables::hittable::HitRecord; //noinspection RsUnusedImport
                                           //Required by enum_dispatch
use crate::math_structures::color::Color; //noinspection RsUnusedImport
                                          //Required by enum_dispatch
use crate::math_structures::ray::Ray; //noinspection RsUnusedImport
                                      //Required by enum_dispatch
use crate::math_structures::vec3::Point3; //Required by enum_dispatch
use enum_dispatch::enum_dispatch;

use crate::materials::material::Material;

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropical;
pub mod lambertian;
pub mod material;
pub mod metal;

#[derive(Clone)]
#[enum_dispatch]
pub enum MatEnum {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
    DiffuseLight(diffuse_light::DiffuseLight),
    Isotropic(isotropical::Isotropic),
}
