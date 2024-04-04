use crate::math_structures::color::foo::fmt_to_file;
use crate::math_structures::interval::Interval;
use crate::math_structures::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    return linear_component.sqrt();
}

pub(crate) mod foo {
    macro_rules! fmt_to_file {
    (&mut $file:expr,$($arg:tt)*) => {
    let res = format!("{}",format_args!($($arg)*));
    $file.write(&*res.into_bytes()).expect("Couldn't Write to File!");
};
}
    pub(crate) use fmt_to_file;
}

pub fn write_color(out_file: &mut File, pixel_color: &Color, samples_per_pixel: i64) {
    let s = write_color_string(pixel_color, samples_per_pixel);
    fmt_to_file!(&mut out_file, "{}", s);
}

pub fn write_color_string(pixel_color: &Color, samples_per_pixel: i64) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero.
    if f64::is_nan(r) {
        r = 0.0
    };
    if f64::is_nan(g) {
        g = 0.0
    };
    if f64::is_nan(b) {
        b = 0.0
    };

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply the linear to gamma transform.
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Write the translated [0,255] value of each color component.
    let intensity = Interval::from(0.000, 0.999);
    let s = format!(
        "{} {} {}\n",
        (256.0 * intensity.clamp(r)) as i64,
        (256.0 * intensity.clamp(g)) as i64,
        (256.0 * intensity.clamp(b)) as i64
    );
    s
}
