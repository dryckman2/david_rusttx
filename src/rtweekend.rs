// Constants

use rand::random;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    random::<f64>()
}

pub fn random_double_bounded(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + ((max - min) * random_double())
}

pub fn random_int_bounded(min: i64, max: i64) -> i64 {
    // Returns a random integer in [min,max].
    random_double_bounded(min as f64, max as f64 + 1.0) as i64
}
