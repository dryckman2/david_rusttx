use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Rgba};

#[derive(Clone)]
pub struct RtwImage {
    img: DynamicImage,
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
        self.img.width() as i64
    }
    pub fn height(&self) -> i64 {
        self.img.height() as i64
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> Rgba<u8> {
        let x = clamp(x as i64, 0, self.width()) as usize;
        let y = clamp(y as i64, 0, self.height()) as usize;
        self.img.get_pixel(x as u32, y as u32)
    }
}

fn load(filename: String) -> Result<RtwImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(filename)?.decode()?;
    Ok(RtwImage { img })
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
