use crate::{Float, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct CFrame {
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
}

impl CFrame {
    pub fn identity() -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: 0.0,
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
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
        }
    }

    pub fn from_columns(x: Vec3, y: Vec3, z: Vec3, p: Vec3) -> Self {
        Self {
            m11: x.x,
            m12: y.x,
            m13: z.x,
            m14: p.x,
            m21: x.y,
            m22: y.y,
            m23: z.y,
            m24: p.y,
            m31: x.z,
            m32: y.z,
            m33: z.z,
            m34: p.z,
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
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
        }
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: pos.x,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: pos.y,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: pos.z,
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
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
        }
    }

    fn vec_axis_angle(n: Vec3, v: Vec3, t: Float) -> Vec3 {
        let n = n.unit();
        let v = v.unit();
        return v * t.cos() + n * v.dot(n) * (1.0 - t.cos()) + n.cross(v) * t.sin();
    }

    pub fn x(&self) -> Vec3 {
        Vec3::new(self.m11, self.m21, self.m31)
    }

    pub fn y(&self) -> Vec3 {
        Vec3::new(self.m12, self.m22, self.m32)
    }

    pub fn z(&self) -> Vec3 {
        Vec3::new(self.m13, self.m23, self.m33)
    }

    pub fn p(&self) -> Vec3 {
        Vec3::new(self.m14, self.m24, self.m34)
    }

    // pub fn from_axis_angle(Vec3 axis, Float theta) {
    // }
}
