use crate::math_structures::vec3::Vec3;
use std::ops::Index;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn blank() -> Onb {
        Onb {
            axis: [Vec3::blank(), Vec3::blank(), Vec3::blank()],
        }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }
    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }
    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn local_from_vec3(&self, a: &Vec3) -> Vec3 {
        &(&(a.x() * self.u()) + &(a.y() * self.v())) + &(a.z() * self.w())
    }

    pub fn build_from_w(&mut self, w: &Vec3) {
        let unit_w = Vec3::unit_vector(w);
        let a = if f64::abs(unit_w.x()) > 0.9 {
            Vec3::from(0.0, 1.0, 0.0)
        } else {
            Vec3::from(1.0, 0.0, 0.0)
        };
        let v = Vec3::unit_vector(&Vec3::cross(&unit_w, &a));
        let u = Vec3::cross(&unit_w, &v);
        self.axis[0] = u;
        self.axis[1] = v;
        self.axis[2] = unit_w;
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
