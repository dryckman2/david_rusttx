use std::f64;
use enum_dispatch::enum_dispatch;

use crate::math_structures::color::Color;
use crate::math_structures::vec3::{Point3};

#[enum_dispatch(TexEnum)]
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

