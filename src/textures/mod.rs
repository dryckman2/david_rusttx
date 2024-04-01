use crate::materials::lambertian::Lambertian;
use crate::materials::{dielectric, diffuse_light, isotropical, lambertian, metal};
use enum_dispatch::enum_dispatch;

pub mod checker_texture;
pub mod image_texture;
mod noise_generation;
pub mod noise_texture;
pub mod solid_color;
pub mod texture;

use crate::math_structures::color::Color;
use crate::math_structures::vec3::Point3;
use crate::textures::texture::Texture;

#[derive(Clone)]
#[enum_dispatch]
pub enum TexEnum {
    CheckerTexture(checker_texture::CheckerTexture),
    ImageTexture(image_texture::ImageTexture),
    NoiseTexture(noise_texture::NoiseTexture),
    SolidColor(solid_color::SolidColor),
}
