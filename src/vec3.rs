use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Float;

#[derive(Clone, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn up() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn down() -> Self {
        Self {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }
    }

    pub fn left() -> Self {
        Self {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn right() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn forward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }
    }

    pub fn backward() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn lerp(mut self, other: &Self, t: Float) -> Self {
        let x = self.x + t * (other.x - self.x);
        let y = self.y + t * (other.y - self.y);
        let z = self.z + t * (other.z - self.z);
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn lerped(&self, other: &Self, t: Float) -> Self {
        let x = self.x + t * (other.x - self.x);
        let y = self.y + t * (other.y - self.y);
        let z = self.z + t * (other.z - self.z);
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(mut self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn crossed(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn normalize(mut self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            self.x *= inv_mag;
            self.y *= inv_mag;
            self.z *= inv_mag;
        }
        self
    }

    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            let x = self.x * inv_mag;
            let y = self.y * inv_mag;
            let z = self.z * inv_mag;
            Self { x, y, z }
        } else {
            Self::zero()
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Self { x, y, z }
    }
}

impl Div<&Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: &Self) -> Self {
        let x = self.x / other.x;
        let y = self.y / other.y;
        let z = self.z / other.z;
        Self { x, y, z }
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: Float) -> Self {
        let x = self.x * scalar;
        let y = self.y * scalar;
        let z = self.z * scalar;
        Self { x, y, z }
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, scalar: Float) -> Self {
        let x = self.x / scalar;
        let y = self.y / scalar;
        let z = self.z / scalar;
        Self { x, y, z }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, scalar: Float) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, scalar: Float) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, other: &Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Self { x, y, z }
    }
}
