use enum_dispatch::enum_dispatch;
use crate::hittables::hittable::HitRecord;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::Point3;

#[enum_dispatch(MatEnum)]
pub trait Material: Clone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        return Color::from(0.0, 0.0, 0.0);
    }
}