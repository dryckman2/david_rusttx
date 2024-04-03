use crate::math_structures::color::Color;
use crate::rtweekend::{random_double, random_double_bounded, PI};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub(crate) e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn blank() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    pub const fn y(&self) -> f64 {
        self.e[1]
    }
    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    // Vector Utility Functions
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::from(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_bounded(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::unit_vector(&Vec3::random_in_unit_sphere());
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        return if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -&on_unit_sphere
        };
    }
    pub fn random() -> Vec3 {
        Vec3::from(random_double(), random_double(), random_double())
    }

    pub fn random_bounded(min: f64, max: f64) -> Vec3 {
        Vec3::from(
            random_double_bounded(min, max),
            random_double_bounded(min, max),
            random_double_bounded(min, max),
        )
    }

    pub fn random_cosine_direction() -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();

        let phi = 2.0 * PI * r1;
        let x = f64::cos(phi) * f64::sqrt(r2);
        let y = f64::sin(phi) * f64::sqrt(r2);
        let z = f64::sqrt(1.0 - r2);

        Vec3::from(x, y, z)
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-uv, n), 1.0);
        let r_out_perp = etai_over_etat * &(uv + &(cos_theta * n));
        let r_out_parallel = -f64::abs(1.0 - r_out_perp.length_squared()).sqrt() * n;
        return &r_out_perp + &r_out_parallel;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - &(2.0 * Vec3::dot(v, n) * n);
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::from(
            random_double_bounded(-1.0, 1.0),
            random_double_bounded(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl Display for &Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.e[0], self.e[1], self.e[2]))
    }
}

//=*=*=*=*=*=*OPTS BELOW THIS POINT*=*=*=*=*=*=
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for &mut Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for &mut Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl AddAssign<&Color> for Vec3 {
    fn add_assign(&mut self, rhs: &Color) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}
