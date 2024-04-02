//noinspection RsUnusedImport
use crate::math_structures::color::Color; //Required
use enum_dispatch::enum_dispatch;

use crate::textures::texture::Texture;

pub mod checker_texture;
pub mod image_texture;
mod noise_generation;
pub mod noise_texture;
pub mod solid_color;
//noinspection RsUnusedImport
use crate::math_structures::vec3::Point3; //Required
pub mod texture;

#[derive(Clone)]
#[enum_dispatch]
pub enum TexEnum {
    CheckerTexture(checker_texture::CheckerTexture),
    ImageTexture(image_texture::ImageTexture),
    NoiseTexture(noise_texture::NoiseTexture),
    SolidColor(solid_color::SolidColor),
}
