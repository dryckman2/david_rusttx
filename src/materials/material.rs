use crate::hittables::hittable::HitRecord;
use crate::math_structures::color::Color;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};
use crate::pdf::cosine_pdf::CosinePdf;
use crate::pdf::pdf::Pdf;
use crate::pdf::sphere_pdf::SpherePdf;
use enum_dispatch::enum_dispatch;

pub struct ScatterRecord {
    pub(crate) attenuation: Color,
    pub(crate) pdf_ptr: Box<dyn Pdf>,
    pub(crate) skip_pdf: bool,
    pub(crate) skip_pdf_ray: Ray,
}

impl ScatterRecord {
    pub fn blank() -> ScatterRecord {
        ScatterRecord {
            attenuation: Vec3::blank(),
            pdf_ptr: Box::new(SpherePdf::blank()),
            skip_pdf: false,
            skip_pdf_ray: Ray::blank(),
        }
    }
}

#[enum_dispatch(MatEnum)]
pub trait Material: Clone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        return Color::from(0.0, 0.0, 0.0);
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}
