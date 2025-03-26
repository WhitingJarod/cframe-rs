use std::{
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{Float, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct CFrame {
    r11: Float,
    r12: Float,
    r13: Float,
    r14: Float,
    r21: Float,
    r22: Float,
    r23: Float,
    r24: Float,
    r31: Float,
    r32: Float,
    r33: Float,
    r34: Float,
}

impl CFrame {
    pub fn x(&self) -> Vec3 {
        Vec3::new(self.r11, self.r21, self.r31)
    }

    pub fn y(&self) -> Vec3 {
        Vec3::new(self.r12, self.r22, self.r32)
    }

    pub fn z(&self) -> Vec3 {
        Vec3::new(self.r13, self.r23, self.r33)
    }

    pub fn p(&self) -> Vec3 {
        Vec3::new(self.r14, self.r24, self.r34)
    }

    pub fn identity() -> Self {
        Self {
            r11: 1.0,
            r12: 0.0,
            r13: 0.0,
            r14: 0.0,
            r21: 0.0,
            r22: 1.0,
            r23: 0.0,
            r24: 0.0,
            r31: 0.0,
            r32: 0.0,
            r33: 1.0,
            r34: 0.0,
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
            r11: m11,
            r12: m12,
            r13: m13,
            r14: m14,
            r21: m21,
            r22: m22,
            r23: m23,
            r24: m24,
            r31: m31,
            r32: m32,
            r33: m33,
            r34: m34,
        }
    }

    pub fn from_columns(x: Vec3, y: Vec3, z: Vec3, p: Vec3) -> Self {
        Self {
            r11: x.x,
            r12: y.x,
            r13: z.x,
            r14: p.x,
            r21: x.y,
            r22: y.y,
            r23: z.y,
            r24: p.y,
            r31: x.z,
            r32: y.z,
            r33: z.z,
            r34: p.z,
        }
    }

    pub fn from_pos_facing(from: Vec3, to: Vec3) -> Self {
        let mut z = (from - to).unit();
        let mut x = Vec3::up().cross(z);
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
        Self {
            r11: x.x,
            r12: y.x,
            r13: z.x,
            r14: from.x,
            r21: x.y,
            r22: y.y,
            r23: z.y,
            r24: from.y,
            r31: x.z,
            r32: y.z,
            r33: z.z,
            r34: from.z,
        }
    }

    pub fn look_at(eye: Vec3, center: Vec3) -> Self {
        if (eye - center).magnitude() == 0.0 {
            return CFrame::identity();
        }
        let z = (eye - center).unit();
        let x = Vec3::up().cross(z).unit();
        if x.magnitude() == 0.0 {
            return CFrame::identity();
        }
        let y = z.cross(x);
        return CFrame {
            r11: x.x,
            r12: y.x,
            r13: z.x,
            r14: -(x.x * eye.x + x.y * eye.y + x.z * eye.z),
            r21: x.y,
            r22: y.y,
            r23: z.y,
            r24: -(y.x * eye.x + y.y * eye.y + y.z * eye.z),
            r31: x.z,
            r32: y.z,
            r33: z.z,
            r34: -(z.x * eye.x + z.y * eye.y + z.z * eye.z),
        };
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self {
            r11: 1.0,
            r12: 0.0,
            r13: 0.0,
            r14: pos.x,
            r21: 0.0,
            r22: 1.0,
            r23: 0.0,
            r24: pos.y,
            r31: 0.0,
            r32: 0.0,
            r33: 1.0,
            r34: pos.z,
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
            r11: m11,
            r12: m12,
            r13: m13,
            r14: m14,
            r21: m21,
            r22: m22,
            r23: m23,
            r24: m24,
            r31: m31,
            r32: m32,
            r33: m33,
            r34: m34,
        }
    }

    pub fn from_axis_angle(axis: Vec3, theta: Float) -> Self {
        let r: Vec3 = Self::vec_axis_angle(axis, Vec3::right(), theta);
        let u: Vec3 = Self::vec_axis_angle(axis, Vec3::up(), theta);
        let b: Vec3 = Self::vec_axis_angle(axis, Vec3::backward(), theta);
        return Self {
            r11: r.x,
            r12: u.x,
            r13: b.x,
            r14: 0.0,
            r21: r.y,
            r22: u.y,
            r23: b.y,
            r24: 0.0,
            r31: r.z,
            r32: u.z,
            r33: b.z,
            r34: 0.0,
        };
    }

    fn vec_axis_angle(mut n: Vec3, v: Vec3, t: Float) -> Vec3 {
        n = n.unit();
        let u = t.cos();
        return v * u + n * v.dot(n) * (1.0 - u) + n.cross(v) * t.sin();
    }

    pub fn perspective(fov: Float, aspect: Float, near: Float, far: Float) -> [Float; 16] {
        let f = 1.0 / (fov / 2.0).tan();
        let c00 = f / aspect;
        let c01 = 0.0;
        let c02 = 0.0;
        let c03 = 0.0;
        let c10 = 0.0;
        let c11 = f;
        let c12 = 0.0;
        let c13 = 0.0;
        let c20 = 0.0;
        let c21 = 0.0;
        let c22 = (far + near) / (near - far);
        let c23 = -1.0;
        let c30 = 0.0;
        let c31 = 0.0;
        let c32 = (2.0 * far * near) / (near - far);
        let c33 = 0.0;
        [
            c00, c01, c02, c03, c10, c11, c12, c13, c20, c21, c22, c23, c30, c31, c32, c33,
        ]
    }

    pub fn to_array(&self) -> [Float; 16] {
        [
            self.r11, self.r21, self.r31, 0.0, self.r12, self.r22, self.r32, 0.0, self.r13,
            self.r23, self.r33, 0.0, self.r14, self.r24, self.r34, 1.0,
        ]
    }

    pub fn determinant(&self) -> Float {
        self.r11 * (self.r22 * self.r33 - self.r32 * self.r23)
            - self.r21 * (self.r12 * self.r33 - self.r32 * self.r13)
            + self.r31 * (self.r12 * self.r23 - self.r22 * self.r13)
    }

    pub fn inverse(&self) -> CFrame {
        let det = self.determinant();
        if det == 0.0 {
            return CFrame::identity();
        }
        let inv_det = 1.0 / det;
        let m11 = (self.r22 * self.r33 - self.r32 * self.r23) * inv_det;
        let m12 = (self.r31 * self.r23 - self.r21 * self.r33) * inv_det;
        let m13 = (self.r21 * self.r32 - self.r31 * self.r22) * inv_det;
        let m14 = (self.r21 * (self.r32 * self.r34 - self.r33 * self.r24)
            + self.r31 * (self.r23 * self.r24 - self.r22 * self.r34)
            + self.r22 * self.r33
            - self.r32 * self.r23)
            * inv_det;
        let m21 = (self.r32 * self.r13 - self.r12 * self.r33) * inv_det;
        let m22 = (self.r11 * self.r33 - self.r31 * self.r13) * inv_det;
        let m23 = (self.r31 * self.r12 - self.r11 * self.r32) * inv_det;
        let m24 = (self.r31 * (self.r12 * self.r34 - self.r13 * self.r24)
            + self.r11 * (self.r23 * self.r24 - self.r22 * self.r34)
            + self.r12 * self.r33
            - self.r32 * self.r13)
            * inv_det;
        let m31 = (self.r12 * self.r23 - self.r22 * self.r13) * inv_det;
        let m32 = (self.r21 * self.r13 - self.r11 * self.r23) * inv_det;
        let m33 = (self.r11 * self.r22 - self.r21 * self.r12) * inv_det;
        let m34 = (self.r11 * (self.r22 * self.r34 - self.r23 * self.r24)
            + self.r21 * (self.r13 * self.r24 - self.r12 * self.r34)
            + self.r12 * self.r23
            - self.r22 * self.r13)
            * inv_det;
        return CFrame {
            r11: m11,
            r12: m12,
            r13: m13,
            r14: m14,
            r21: m21,
            r22: m22,
            r23: m23,
            r24: m24,
            r31: m31,
            r32: m32,
            r33: m33,
            r34: m34,
        };
    }
}

impl Add<Vec3> for CFrame {
    type Output = CFrame;

    fn add(self, rhs: Vec3) -> CFrame {
        CFrame::from_components(
            self.r11,
            self.r12,
            self.r13,
            self.r14 + rhs.x,
            self.r21,
            self.r22,
            self.r23,
            self.r24 + rhs.y,
            self.r31,
            self.r32,
            self.r33,
            self.r34 + rhs.z,
        )
    }
}
impl AddAssign<Vec3> for CFrame {
    fn add_assign(&mut self, rhs: Vec3) {
        self.r14 += rhs.x;
        self.r24 += rhs.y;
        self.r34 += rhs.z;
    }
}

impl Sub<Vec3> for CFrame {
    type Output = CFrame;

    fn sub(self, rhs: Vec3) -> CFrame {
        CFrame::from_components(
            self.r11,
            self.r12,
            self.r13,
            self.r14 - rhs.x,
            self.r21,
            self.r22,
            self.r23,
            self.r24 - rhs.y,
            self.r31,
            self.r32,
            self.r33,
            self.r34 - rhs.z,
        )
    }
}
impl SubAssign<Vec3> for CFrame {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.r14 -= rhs.x;
        self.r24 -= rhs.y;
        self.r34 -= rhs.z;
    }
}

impl Mul for CFrame {
    type Output = CFrame;

    fn mul(self, rhs: CFrame) -> CFrame {
        CFrame::from_components(
            self.r11 * rhs.r11 + self.r12 * rhs.r21 + self.r13 * rhs.r31,
            self.r11 * rhs.r12 + self.r12 * rhs.r22 + self.r13 * rhs.r32,
            self.r11 * rhs.r13 + self.r12 * rhs.r23 + self.r13 * rhs.r33,
            self.r11 * rhs.r14 + self.r12 * rhs.r24 + self.r13 * rhs.r34 + self.r14,
            self.r21 * rhs.r11 + self.r22 * rhs.r21 + self.r23 * rhs.r31,
            self.r21 * rhs.r12 + self.r22 * rhs.r22 + self.r23 * rhs.r32,
            self.r21 * rhs.r13 + self.r22 * rhs.r23 + self.r23 * rhs.r33,
            self.r21 * rhs.r14 + self.r22 * rhs.r24 + self.r23 * rhs.r34 + self.r24,
            self.r31 * rhs.r11 + self.r32 * rhs.r21 + self.r33 * rhs.r31,
            self.r31 * rhs.r12 + self.r32 * rhs.r22 + self.r33 * rhs.r32,
            self.r31 * rhs.r13 + self.r32 * rhs.r23 + self.r33 * rhs.r33,
            self.r31 * rhs.r14 + self.r32 * rhs.r24 + self.r33 * rhs.r34 + self.r34,
        )
    }
}
impl MulAssign for CFrame {
    fn mul_assign(&mut self, rhs: CFrame) {
        *self = *self * rhs;
    }
}

impl fmt::Debug for CFrame {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "CFrame {{ x: {:?}, y: {:?}, z: {:?}, p: {:?} }}",
            self.x(),
            self.y(),
            self.z(),
            self.p()
        )
    }
}
