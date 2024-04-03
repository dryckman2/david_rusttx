use crate::hittables::hittable::HitRecord;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Point3;
use enum_dispatch::enum_dispatch;
use crate::rtweekend::PI;

#[enum_dispatch(MatEnum)]
pub trait Material: Clone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord,pdf:f64) -> Option<(Color, Ray,f64)>;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        return Color::from(0.0, 0.0, 0.0);
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord)->f64{
      0.0
    }
}
