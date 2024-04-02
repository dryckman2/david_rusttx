use crate::math_structures::vec3::{Point3, Vec3};
use crate::rtweekend::random_int_bounded;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i64; POINT_COUNT],
    perm_y: [i64; POINT_COUNT],
    perm_z: [i64; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = [Vec3::blank(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranvec[i] = Vec3::unit_vector(&Vec3::random_bounded(-1.0, 1.0));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c = [[[Vec3::blank(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x
                        [(i as u128 + di as u128) as usize & 255]
                        ^ self.perm_y[(j as u128 + dj as u128) as usize & 255]
                        ^ self.perm_z[(k as u128 + dk as u128) as usize & 255])
                        as usize];
                }
            }
        }
        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3) -> f64 {
        self.turb_set_depth(p, 7)
    }
    pub fn turb_set_depth(&self, p: &Point3, depth: i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = &temp_p * 2.0;
        }

        f64::abs(accum)
    }
}

fn perlin_generate_perm() -> [i64; POINT_COUNT] {
    let mut p = [0i64; POINT_COUNT];
    for i in 0..POINT_COUNT {
        p[i] = i as i64;
    }
    permute(&mut p, POINT_COUNT);
    p
}

fn permute(p: &mut [i64; 256], n: usize) {
    for i in (0..(n - 1)).rev() {
        let target = random_int_bounded(0, i as i64) as usize;
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

pub fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::from(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * Vec3::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}
