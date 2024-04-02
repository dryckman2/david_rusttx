use crate::math_structures::color::Color;
use crate::math_structures::interval::Interval;
use crate::math_structures::vec3::Point3;
use crate::rtw_image::RtwImage;
use crate::textures::texture::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn from(filename: &str) -> ImageTexture {
        ImageTexture {
            image: RtwImage::from_image(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {
            return Color::from(0.0, 1.0, 1.0);
        };
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::from(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::from(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        let r = color_scale * pixel[0] as f64;
        let g = color_scale * pixel[1] as f64;
        let b = color_scale * pixel[2] as f64;

        return Color::from(r, g, b);
    }
}
