use image::io::Reader as ImageReader;
use image::{ GenericImageView};
use crate::math_structures::color::Color;

#[derive(Clone)]
pub struct RtwImage {
    width: i64,
    height:i64,
    img: Vec<Vec<Color>>,
}

impl RtwImage {
    pub fn from_image(image_filename: &str) -> RtwImage {
        let full_file_name = "./images/".to_string() + image_filename;
        match { load(full_file_name) } {
            Ok(x) => {
                return x;
            }
            Err(e) => {
                panic!("Couldn't Open Image: {}", e)
            }
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }
    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> Color {
        let x = clamp(x as i64, 0, self.width()) as usize;
        let y = clamp(y as i64, 0, self.height()) as usize;
        self.img[x][y]
    }
}

fn load(filename: String) -> Result<RtwImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(filename)?.decode()?;
    let image_width = img.width();
    let image_height = img.height();
    let mut image = vec![];
    for x in 0..image_width{
        let mut new_row = vec![];
        for y in 0..image_height{
        let pixel = img.get_pixel(x,y);

            let color_scale = 1.0 / 255.0;
            let r = color_scale * pixel[0] as f64;
            let g = color_scale * pixel[1] as f64;
            let b = color_scale * pixel[2] as f64;
            new_row.push(Color::from(r,g,b));
        }
        image.push(new_row);
    }
    Ok(RtwImage { width: image_width as i64, height: image_height as i64, img:  image })
}

pub fn clamp(x: i64, low: i64, high: i64) -> i64 {
    // Return the value clamped to the range [low, high).
    return if x < low {
        low
    } else if x < high {
        x
    } else {
        high - 1
    };
}
