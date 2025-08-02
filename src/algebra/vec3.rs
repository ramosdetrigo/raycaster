use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};
use wide::f64x4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    data: f64x4,
}
unsafe impl Send for Vec3 {}

impl Vec3 {
    #[inline]
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            data: f64x4::new([x, y, z, 0.0]),
        }
    }

    #[inline]
    #[must_use]
    pub fn splat(n: f64) -> Vec3 {
        Vec3 {
            data: f64x4::new([n, n, n, 0.0]),
        }
    }

    pub fn x(&self) -> f64 {
        self.data.as_array_ref()[0]
    }

    pub fn y(&self) -> f64 {
        self.data.as_array_ref()[1]
    }

    pub fn z(&self) -> f64 {
        self.data.as_array_ref()[2]
    }

    #[inline]
    #[must_use]
    /// Vector dot product
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.data * rhs.data).reduce_add()
    }

    #[must_use]
    /// Vector cross product
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        let a = self.data.as_array_ref();
        let b = rhs.data.as_array_ref();

        let a_yzx = f64x4::new([a[1], a[2], a[0], 0.0]);
        let b_yzx = f64x4::new([b[1], b[2], b[0], 0.0]);

        let a_zxy = f64x4::new([a[2], a[0], a[1], 0.0]);
        let b_zxy = f64x4::new([b[2], b[0], b[1], 0.0]);

        Vec3 {
            data: a_yzx * b_zxy - a_zxy * b_yzx,
        }
    }

    #[inline]
    #[must_use]
    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    #[inline]
    #[must_use]
    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalized(&self) -> Vec3 {
        *self / self.magnitude()
    }

    #[inline]
    #[must_use]
    pub fn rgb_normal(&self) -> Vec3 {
        *self / 255.0
    }

    #[inline]
    #[must_use]
    pub fn rgb_255(&self) -> Vec3 {
        *self * 255.0
    }

    #[inline]
    #[must_use]
    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3 {
            data: self
                .data
                .fast_min(f64x4::splat(min))
                .fast_max(f64x4::splat(max)),
        }
    }
}

/// Vector * Scalar
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            data: self.data * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data *= f64x4::splat(rhs)
    }
}

/// Scalar * Vector
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            data: self * rhs.data,
        }
    }
}

/// Element-wise multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            data: self.data * rhs.data,
        }
    }
}

/// Element-wise multiplication
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.data *= rhs.data
    }
}

/// Vector addition
impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            data: self.data + rhs.data,
        }
    }
}

/// Vector addition
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.data += rhs.data
    }
}

/// Vector / Scalar
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            data: self.data / rhs,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self::Output {
        let mut data = self.data / rhs.data;
        data.as_array_mut()[3] = 0.0;
        Self { data }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.data /= f64x4::splat(rhs)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.data /= rhs.data;
        self.data.as_array_mut()[3] = 0.0
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { data: -self.data }
    }
}
