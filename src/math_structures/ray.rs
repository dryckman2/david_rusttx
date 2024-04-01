use crate::math_structures::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn from(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            tm: 0.0,
        }
    }

    pub fn from_set_time(origin: Point3, direction: Vec3, tm: f64) -> Ray {
        Ray {
            origin,
            direction,
            tm,
        }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub const fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(t * (&self.direction))
    }
}
