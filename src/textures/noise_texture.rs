use crate::math_structures::color::Color;
use crate::textures::texture::Texture;
use crate::math_structures::vec3::{Point3};
use crate::textures::noise_generation::perlin::Perlin;

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let s = self.scale * p;
        return &(&Color::from(1.0, 1.0, 1.0) * 0.5) * (1.0 + f64::sin(s.z() + 10.0 * self.noise.turb(&s)));
    }
}
