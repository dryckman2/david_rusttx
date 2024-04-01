use enum_dispatch::enum_dispatch;
use crate::materials::{dielectric, diffuse_light, isotropical, lambertian, metal};
use crate::materials::lambertian::Lambertian;

pub mod texture;
pub mod solid_color;
pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;
mod noise_generation;

use crate::textures::texture::Texture;
use crate::math_structures::vec3::Point3;
use crate::math_structures::color::Color;


#[derive(Clone)]
#[enum_dispatch]
pub enum TexEnum {
    CheckerTexture(checker_texture::CheckerTexture),
    ImageTexture(image_texture::ImageTexture),
    NoiseTexture(noise_texture::NoiseTexture),
    SolidColor(solid_color::SolidColor),
}