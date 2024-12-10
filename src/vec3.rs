use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use crate::utils::{random_between, random_f64};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn with_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn random() -> Vec3 {
        Vec3::with_values(random_f64(), random_f64(), random_f64())
    }

    pub fn random_min_max(min: f64, max: f64) -> Vec3 {
        Vec3::with_values(
            random_between(min, max),
            random_between(min, max),
            random_between(min, max),
        )
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_unit_vector();

        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        }

        -in_unit_sphere
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;

        r_out_perp + r_out_parallel
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        return self[0] * v[0] + self[1] * v[1] + self[2] * v[2];
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        return Vec3::with_values(
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0],
        );
    }

    pub fn unit_vector(&self) -> Vec3 {
        return *self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self[0].abs() < s && self[1].abs() < s && self[2].abs() < s;
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let v = Vec3::random_min_max(-1.0, 1.0);
            let lensq = v.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return v / lensq;
            }
        }
    }

    pub fn reflect(vec: Vec3, normal: &Vec3) -> Vec3 {
        return vec - 2.0 * vec.dot(normal) * *normal;
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::with_values(random_between(-1.0, 1.0), random_between(-1.0, 1.0), 0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::with_values(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::with_values(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::with_values(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::with_values(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::with_values(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}
