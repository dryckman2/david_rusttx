use crate::math_structures::color::Color;
use crate::math_structures::vec3::Point3;
use crate::textures::solid_color::SolidColor;
use crate::textures::texture::Texture;
use crate::textures::TexEnum;

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<TexEnum>,
    odd: Box<TexEnum>,
}

impl CheckerTexture {
    pub fn from_textures(scale: f64, even: Box<TexEnum>, odd: Box<TexEnum>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Box::new(TexEnum::SolidColor(SolidColor::from(c1))),
            odd: Box::new(TexEnum::SolidColor(SolidColor::from(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = f64::floor(self.inv_scale * p.x()) as i64;
        let y_integer = f64::floor(self.inv_scale * p.y()) as i64;
        let z_integer = f64::floor(self.inv_scale * p.z()) as i64;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        return if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        };
    }
}
