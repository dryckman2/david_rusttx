use crate::hittables::hittable::HitRecord;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Point3;

use crate::materials::material::{Material, ScatterRecord};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropical;
pub mod lambertian;
pub mod material;
pub mod metal;

#[derive(Clone)]
pub enum MatEnum {
    Default(DefaultMat),
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
    DiffuseLight(diffuse_light::DiffuseLight),
    Isotropic(isotropical::Isotropic),
}

#[derive(Clone)]
pub struct DefaultMat {}

impl Material for MatEnum {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let x;
        match self {
            MatEnum::Default(y) => {
                panic!("Default Material Should Not Scatter!")
            }
            MatEnum::Lambertian(y) => {
                x = y.scatter(r_in, rec);
            }
            MatEnum::Metal(y) => {
                x = y.scatter(r_in, rec);
            }
            MatEnum::Dielectric(y) => {
                x = y.scatter(r_in, rec);
            }
            MatEnum::DiffuseLight(y) => {
                x = y.scatter(r_in, rec);
            }
            MatEnum::Isotropic(y) => {
                x = y.scatter(r_in, rec);
            }
        }
        x
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        let x;
        match self {
            MatEnum::Default(y) => {
                panic!("Default Material Should Not Emit!")
            }
            MatEnum::Lambertian(y) => {
                x = y.emitted(r_in, rec, u, v, p);
            }
            MatEnum::Metal(y) => {
                x = y.emitted(r_in, rec, u, v, p);
            }
            MatEnum::Dielectric(y) => {
                x = y.emitted(r_in, rec, u, v, p);
            }
            MatEnum::DiffuseLight(y) => {
                x = y.emitted(r_in, rec, u, v, p);
            }
            MatEnum::Isotropic(y) => {
                x = y.emitted(r_in, rec, u, v, p);
            }
        }
        x
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let x;
        match self {
            MatEnum::Default(y) => {
                panic!("Default Material Should Not Scatter PDF!")
            }
            MatEnum::Lambertian(y) => {
                x = y.scattering_pdf(r_in, rec, scattered);
            }
            MatEnum::Metal(y) => {
                x = y.scattering_pdf(r_in, rec, scattered);
            }
            MatEnum::Dielectric(y) => {
                x = y.scattering_pdf(r_in, rec, scattered);
            }
            MatEnum::DiffuseLight(y) => {
                x = y.scattering_pdf(r_in, rec, scattered);
            }
            MatEnum::Isotropic(y) => {
                x = y.scattering_pdf(r_in, rec, scattered);
            }
        }
        x
    }
}
