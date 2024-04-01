use crate::math_structures::color::Color;
use crate::textures::texture::Texture;
use crate::math_structures::vec3::{Point3};

#[derive(Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn from(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_value: Color::from(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}
