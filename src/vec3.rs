use std::ops;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
    Vec3{e: [e0, e1, e2]}
}

pub fn unit_vec(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn one() -> Vec3 {
    Vec3 {e: [1.0, 1.0, 1.0] }
}

#[allow(dead_code)]
impl Vec3 {
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3{ e: [(v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1]),
            (-(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0])),
            (v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]) ]
        }
    }

    pub fn length(&self) -> f64 {
       f64::sqrt(self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "e0: {}, e1: {}, e2: {}", self.e[0],self.e[1], self.e[2])
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Vec3 {
        self.e[0] += rhs.e[0];self.e[1] += rhs.e[1];self.e[2] += rhs.e[2];
        self
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]] };
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, t: f64) {
        *self = Self { e: [self.e[0] + t, self.e[1] + t, self.e[2] + t] };
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] -= rhs.e[0];self.e[1] -= rhs.e[1];self.e[2] -= rhs.e[2];
        self
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]] };
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.e[0] /= rhs.e[0];self.e[1] /= rhs.e[1];self.e[2] /= rhs.e[2];
        self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(mut self, t: f64) -> Self::Output {
        self.e[0] /= t;self.e[1] /= t;self.e[2] /= t;
        self
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]] };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Self { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.e[0] *= rhs.e[0];self.e[1] *= rhs.e[1];self.e[2] *= rhs.e[2];
        self
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, t: f64) -> Self::Output {
        self.e[0] *= t;self.e[1] *= t;self.e[2] *= t;
        self
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs.e[0] *= self;rhs.e[1] *= self;rhs.e[2] *= self;
        rhs
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]] };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = new(0.0, 0.0, 0.0);
        let v2 = new(1.0, 1.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v1.e[0] + v2.e[0], v3.e[0]);
        assert_eq!(v1.e[1] + v2.e[1], v3.e[1]);
        assert_eq!(v1.e[2] + v2.e[2], v3.e[2]);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = new(0.0, 0.0, 0.0);
        v1 += new(1.0, 1.0, 1.0);
        assert_eq!(v1.e[0], 1.0);
        assert_eq!(v1.e[1], 1.0);
        assert_eq!(v1.e[2], 1.0);
    }

    #[test]
    fn test_sub() {
        let v1 = new(0.0, 0.0, 0.0);
        let v2 = new(1.0, 1.0, 1.0);
        let v3 = v1 - v2;
        assert_eq!(v1.e[0] - v2.e[0], v3.e[0]);
        assert_eq!(v1.e[1] - v2.e[1], v3.e[1]);
        assert_eq!(v1.e[2] - v2.e[2], v3.e[2]);
    }

    #[test]
    fn test_mul() {
        let v1 = new(0.0, 0.0, 0.0);
        let v2 = new(1.0, 1.0, 1.0);
        let v3 = v1 * v2;
        assert_eq!(v1.e[0] * v2.e[0], v3.e[0]);
        assert_eq!(v1.e[1] * v2.e[1], v3.e[1]);
        assert_eq!(v1.e[2] * v2.e[2], v3.e[2]);
    }

    #[test]
    fn test_mul2() {
        let v1 = new(1.0, 0.0, 3.0);
        let t = 5.0;
        let v3 = t * v1;
        assert_eq!(v3.e[0], 5.0);
        assert_eq!(v3.e[1], 0.0);
        assert_eq!(v3.e[2], 15.0);
    }

    #[test]
    fn test_div() {
        let v1 = new(0.0, 0.0, 0.0);
        let v2 = new(1.0, 1.0, 1.0);
        let v3 = v1 / v2;
        assert_eq!(v1.e[0] / v2.e[0], v3.e[0]);
        assert_eq!(v1.e[1] / v2.e[1], v3.e[1]);
        assert_eq!(v1.e[2] / v2.e[2], v3.e[2]);
    }
}