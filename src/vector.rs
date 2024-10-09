use std::ops;

use crate::util::{random_between, random_double};

#[derive(Clone, Copy, Debug)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3([e1, e2, e3])
    }

    pub fn random() -> Vec3 {
        Vec3([random_double(), random_double(), random_double()])
    }

    pub fn random_between(min: f64, max: f64) -> Vec3 {
        Vec3([random_between(min, max), random_between(min, max), random_between(min, max)])
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_between(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p.unit_vector();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let unit = Self::random_unit_vector();
        if Self::dot(unit, *normal) > 0.0 {
            unit
        } else {
            -unit
        }
    }

    pub fn zero() -> Vec3 {
        Vec3([0.0, 0.0, 0.0])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3::new(self[0] / self.length(), self[1] / self.length(), self[2] / self.length())
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
       v1[0] * v2[0] + v1[1] * v2[1] + v2[2] * v2[2]
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3::new(v1[0] * v2[2] - v1[2] * v2[1], v1[2] * v2[0] - v1[0] * v2[2], v1[0] * v2[1] - v1[1] * v2[0])
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2])
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::MulAssign<isize> for Vec3 {
    fn mul_assign(&mut self, rhs: isize) {
        *self *= rhs as f64;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<isize> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: isize) -> Self::Output {
        self * rhs as f64
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl ops::DivAssign<isize> for Vec3 {
    fn div_assign(&mut self, rhs: isize) {
        *self /= rhs as f64;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0]/rhs, self[1]/rhs, self[2]/rhs)
    }
}

impl ops::Div<isize> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: isize) -> Self::Output {
        self / rhs as f64
    }
}
