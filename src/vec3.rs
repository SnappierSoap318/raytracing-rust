use std::f64;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other:Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(self, other:Self) -> Vec3 {
        Vec3 {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }
    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();
        self/len
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}
//Scalar Multiplication
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}
// Vector Multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Vec3) -> Self {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

// Scalar multiplication commutative
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

// Implement Multiply and Assign for Scalar
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

// Scalar Division
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self::Output {
        Self {
            x: self.x * 1.0 / _rhs,
            y: self.y * 1.0 / _rhs,
            z: self.z * 1.0 / _rhs,
        }
    }
}
// Vector Division
impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * 1.0 / _rhs.x,
            y: self.y * 1.0 / _rhs.y,
            z: self.z * 1.0 / _rhs.z,
        }
    }
}