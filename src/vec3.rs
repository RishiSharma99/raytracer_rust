use std::fmt::Display;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn len(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[inline]
    pub fn len_sqrd(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[inline]
pub const fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
}

#[inline]
pub fn norm(p: Vec3) -> Vec3 {
    let len = len(p);
    Vec3::new(p.x / len, p.y / len, p.z / len)
}

#[inline]
pub fn len(p: Vec3) -> f64 {
    p.len()
}

pub const fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.x,
        lhs.x * rhs.y - lhs.y * rhs.z,
    )
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(value: Vec3) -> Self {
        (value.x, value.y, value.z)
    }
}

// -------------------------------------
// Add
// -------------------------------------
impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        self + (*rhs)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self - (*rhs)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        rhs - self
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<&f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for &f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn lerp(start_value: &Vec3, end_value: &Vec3, t: f64) -> Vec3 {
    ((1.0 - t) * start_value) + (t * end_value)
}
