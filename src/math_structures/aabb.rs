use std::mem::swap;
use std::ops::Add;

use crate::math_structures::interval::Interval;
use crate::math_structures::ray::Ray;
use crate::math_structures::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Aabb {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl Aabb {
    pub fn blank() -> Aabb {
        Aabb {
            x: Interval::blank(),
            y: Interval::blank(),
            z: Interval::blank(),
        }
    }

    pub fn from(ix: Interval, iy: Interval, iz: Interval) -> Aabb {
        Aabb {
            x: ix,
            y: iy,
            z: iz,
        }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Aabb {
        Aabb {
            x: Interval::from(f64::min(a[0], b[0]), f64::max(a[0], b[0])),
            y: Interval::from(f64::min(a[1], b[1]), f64::max(a[1], b[1])),
            z: Interval::from(f64::min(a[2], b[2]), f64::max(a[2], b[2])),
        }
    }

    pub fn from_aabbs(box0: &Aabb, box1: &Aabb) -> Aabb {
        let x = Interval::from_intervals(&box0.x, &box1.x);
        let y = Interval::from_intervals(&box0.y, &box1.y);
        let z = Interval::from_intervals(&box0.z, &box1.z);
        Aabb { x, y, z }
    }

    pub fn axis(&self, n: i64) -> Interval {
        return if n == 1 {
            self.y.clone()
        } else if n == 2 {
            self.z.clone()
        } else {
            self.x.clone()
        };
    }

    pub fn hit(&self, r: &Ray) -> Option<Interval> {
        let mut ray_t = Interval::blank();
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a as i64).min - orig) * inv_d;
            let mut t1 = (self.axis(a as i64).max - orig) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return None;
            }
        }
        return Some(ray_t);
    }

    pub fn pad(&self) -> Aabb {
        // Return an AABB that has no side narrower than some delta, padding if necessary.
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x.clone()
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y.clone()
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z.clone()
        } else {
            self.z.expand(delta)
        };

        Aabb::from(new_x, new_y, new_z)
    }
}

impl Add<&Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Aabb::from(&self.x + rhs.x(), &self.y + rhs.y(), &self.z + rhs.z())
    }
}

impl Add<&Aabb> for &Vec3 {
    type Output = Aabb;

    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}
