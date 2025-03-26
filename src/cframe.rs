use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::{Float, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct CFrame {
    c00: Float,
    c10: Float,
    c20: Float,
    c30: Float,
    c01: Float,
    c11: Float,
    c21: Float,
    c31: Float,
    c02: Float,
    c12: Float,
    c22: Float,
    c32: Float,
}

impl CFrame {
    pub fn x(&self) -> Vec3 {
        Vec3::new(self.c00, self.c01, self.c02)
    }

    pub fn y(&self) -> Vec3 {
        Vec3::new(self.c10, self.c11, self.c12)
    }

    pub fn z(&self) -> Vec3 {
        Vec3::new(self.c20, self.c21, self.c22)
    }

    pub fn p(&self) -> Vec3 {
        Vec3::new(self.c30, self.c31, self.c32)
    }

    pub fn identity() -> Self {
        Self {
            c00: 1.0,
            c10: 0.0,
            c20: 0.0,
            c30: 0.0,
            c01: 0.0,
            c11: 1.0,
            c21: 0.0,
            c31: 0.0,
            c02: 0.0,
            c12: 0.0,
            c22: 1.0,
            c32: 0.0,
        }
    }

    pub fn from_components(
        m11: Float,
        m12: Float,
        m13: Float,
        m14: Float,
        m21: Float,
        m22: Float,
        m23: Float,
        m24: Float,
        m31: Float,
        m32: Float,
        m33: Float,
        m34: Float,
    ) -> Self {
        Self {
            c00: m11,
            c10: m12,
            c20: m13,
            c30: m14,
            c01: m21,
            c11: m22,
            c21: m23,
            c31: m24,
            c02: m31,
            c12: m32,
            c22: m33,
            c32: m34,
        }
    }

    pub fn from_columns(x: Vec3, y: Vec3, z: Vec3, p: Vec3) -> Self {
        Self {
            c00: x.x,
            c10: y.x,
            c20: z.x,
            c30: p.x,
            c01: x.y,
            c11: y.y,
            c21: z.y,
            c31: p.y,
            c02: x.z,
            c12: y.z,
            c22: z.z,
            c32: p.z,
        }
    }

    pub fn from_pos_facing(from: Vec3, to: Vec3) -> Self {
        let mut z = (to - from).unit();
        let mut x = Vec3::up().cross(z).unit();
        let mut y = z.cross(x);
        if x.magnitude() == 0.0 {
            if z.y < 0.0 {
                x = Vec3::forward();
                y = Vec3::right();
                z = Vec3::down();
            } else {
                x = Vec3::backward();
                y = Vec3::right();
                z = Vec3::up();
            }
        }
        let m11 = x.x;
        let m12 = y.x;
        let m13 = z.x;
        let m14 = from.x;
        let m21 = x.y;
        let m22 = y.y;
        let m23 = z.y;
        let m24 = from.y;
        let m31 = x.z;
        let m32 = y.z;
        let m33 = z.z;
        let m34 = from.z;

        Self {
            c00: m11,
            c10: m12,
            c20: m13,
            c30: m14,
            c01: m21,
            c11: m22,
            c21: m23,
            c31: m24,
            c02: m31,
            c12: m32,
            c22: m33,
            c32: m34,
        }
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self {
            c00: 1.0,
            c10: 0.0,
            c20: 0.0,
            c30: pos.x,
            c01: 0.0,
            c11: 1.0,
            c21: 0.0,
            c31: pos.y,
            c02: 0.0,
            c12: 0.0,
            c22: 1.0,
            c32: pos.z,
        }
    }

    pub fn from_pos_quaternions(pos: Vec3, i: Float, j: Float, k: Float, w: Float) -> Self {
        let m14 = pos.x;
        let m24 = pos.y;
        let m34 = pos.z;
        let m11 = 1.0 - 2.0 * (j * j - k * k);
        let m12 = 2.0 * (i * j - k * w);
        let m13 = 2.0 * (i * k + j * w);
        let m21 = 2.0 * (i * j + k * w);
        let m22 = 1.0 - 2.0 * (i * i - k * k);
        let m23 = 2.0 * (j * k - i * w);
        let m31 = 2.0 * (i * k - j * w);
        let m32 = 2.0 * (j * k + i * w);
        let m33 = 1.0 - 2.0 * (i * i - j * j);

        Self {
            c00: m11,
            c10: m12,
            c20: m13,
            c30: m14,
            c01: m21,
            c11: m22,
            c21: m23,
            c31: m24,
            c02: m31,
            c12: m32,
            c22: m33,
            c32: m34,
        }
    }

    pub fn from_axis_angle(axis: Vec3, theta: Float) -> Self {
        let r: Vec3 = Self::vec_axis_angle(axis, Vec3::right(), theta);
        let u: Vec3 = Self::vec_axis_angle(axis, Vec3::up(), theta);
        let b: Vec3 = Self::vec_axis_angle(axis, Vec3::forward(), theta);
        return Self {
            c00: r.x,
            c10: u.x,
            c20: b.x,
            c30: 0.0,
            c01: r.y,
            c11: u.y,
            c21: b.y,
            c31: 0.0,
            c02: r.z,
            c12: u.z,
            c22: b.z,
            c32: 0.0,
        };
    }

    fn vec_axis_angle(mut n: Vec3, mut v: Vec3, t: Float) -> Vec3 {
        n = n.unit();
        v = v.unit();
        let u = t.cos();
        return v * u + n * v.dot(n) * (1.0 - u) + n.cross(v) * t.sin();
    }

    pub fn perspective(fov: Float, aspect: Float, near: Float, far: Float) -> [Float; 16] {
        let f = 1.0 / (fov / 2.0).tan();
        let c00 = f / aspect;
        let c11 = f;
        let nf = near - far;
        let c22 = (far + near) / nf;
        let c32 = 2.0 * far * near / nf;
        return [
            c00, 0.0, 0.0, 0.0, 0.0, c11, 0.0, 0.0, 0.0, 0.0, c22, -1.0, 0.0, 0.0, c32, 0.0,
        ];
    }

    pub fn to_array(&self) -> [Float; 16] {
        [
            self.c00, self.c10, self.c20, self.c30, self.c01, self.c11, self.c21, self.c31,
            self.c02, self.c12, self.c22, self.c32, 0.0, 0.0, 0.0, 1.0,
        ]
    }

    pub fn determinant(&self) -> Float {
        self.c00 * (self.c11 * self.c22 - self.c12 * self.c21)
            - self.c01 * (self.c10 * self.c22 - self.c12 * self.c20)
            + self.c02 * (self.c10 * self.c21 - self.c11 * self.c20)
    }

    pub fn inverse(&self) -> CFrame {
        let det = self.determinant();
        if det == 0.0 {
            return CFrame::identity();
        }
        let inv_det = 1.0 / det;
        let m11 = (self.c11 * self.c22 - self.c12 * self.c21) * inv_det;
        let m12 = (self.c02 * self.c21 - self.c01 * self.c22) * inv_det;
        let m13 = (self.c01 * self.c12 - self.c02 * self.c11) * inv_det;
        let m14 = (self.c01 * (self.c12 * self.c32 - self.c22 * self.c31)
            + self.c02 * (self.c21 * self.c31 - self.c11 * self.c32)
            + self.c11 * self.c22
            - self.c12 * self.c21)
            * inv_det;
        let m21 = (self.c12 * self.c20 - self.c10 * self.c22) * inv_det;
        let m22 = (self.c00 * self.c22 - self.c02 * self.c20) * inv_det;
        let m23 = (self.c02 * self.c10 - self.c00 * self.c12) * inv_det;
        let m24 = (self.c02 * (self.c10 * self.c32 - self.c20 * self.c31)
            + self.c00 * (self.c21 * self.c31 - self.c11 * self.c32)
            + self.c10 * self.c22
            - self.c12 * self.c20)
            * inv_det;
        let m31 = (self.c10 * self.c21 - self.c11 * self.c20) * inv_det;
        let m32 = (self.c01 * self.c20 - self.c00 * self.c21) * inv_det;
        let m33 = (self.c00 * self.c11 - self.c01 * self.c10) * inv_det;
        let m34 = (self.c00 * (self.c11 * self.c32 - self.c21 * self.c31)
            + self.c01 * (self.c20 * self.c31 - self.c10 * self.c32)
            + self.c10 * self.c21
            - self.c11 * self.c20)
            * inv_det;
        return CFrame {
            c00: m11,
            c10: m12,
            c20: m13,
            c30: m14,
            c01: m21,
            c11: m22,
            c21: m23,
            c31: m24,
            c02: m31,
            c12: m32,
            c22: m33,
            c32: m34,
        };
    }
}

impl Add<Vec3> for CFrame {
    type Output = CFrame;

    fn add(self, rhs: Vec3) -> CFrame {
        CFrame::from_components(
            self.c00,
            self.c10,
            self.c20,
            self.c30 + rhs.x,
            self.c01,
            self.c11,
            self.c21,
            self.c31 + rhs.y,
            self.c02,
            self.c12,
            self.c22,
            self.c32 + rhs.z,
        )
    }
}
impl AddAssign<Vec3> for CFrame {
    fn add_assign(&mut self, rhs: Vec3) {
        self.c30 += rhs.x;
        self.c31 += rhs.y;
        self.c32 += rhs.z;
    }
}

impl Mul for CFrame {
    type Output = CFrame;

    fn mul(self, rhs: CFrame) -> CFrame {
        CFrame::from_components(
            self.c00 * rhs.c00 + self.c10 * rhs.c01 + self.c20 * rhs.c02,
            self.c00 * rhs.c10 + self.c10 * rhs.c11 + self.c20 * rhs.c12,
            self.c00 * rhs.c20 + self.c10 * rhs.c21 + self.c20 * rhs.c22,
            self.c00 * rhs.c30 + self.c10 * rhs.c31 + self.c20 * rhs.c32 + self.c30,
            self.c01 * rhs.c00 + self.c11 * rhs.c01 + self.c21 * rhs.c02,
            self.c01 * rhs.c10 + self.c11 * rhs.c11 + self.c21 * rhs.c12,
            self.c01 * rhs.c20 + self.c11 * rhs.c21 + self.c21 * rhs.c22,
            self.c01 * rhs.c30 + self.c11 * rhs.c31 + self.c21 * rhs.c32 + self.c31,
            self.c02 * rhs.c00 + self.c12 * rhs.c01 + self.c22 * rhs.c02,
            self.c02 * rhs.c10 + self.c12 * rhs.c11 + self.c22 * rhs.c12,
            self.c02 * rhs.c20 + self.c12 * rhs.c21 + self.c22 * rhs.c22,
            self.c02 * rhs.c30 + self.c12 * rhs.c31 + self.c22 * rhs.c32 + self.c32,
        )
    }
}
impl MulAssign for CFrame {
    fn mul_assign(&mut self, rhs: CFrame) {
        *self = *self * rhs;
    }
}
