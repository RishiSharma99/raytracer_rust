use crate::vec3;

pub type Rgb = vec3::Vec3;

impl Rgb {
    #[inline]
    pub const fn r(&self) -> f64 {
        self.x
    }

    #[inline]
    pub const fn g(&self) -> f64 {
        self.y
    }

    #[inline]
    pub const fn b(&self) -> f64 {
        self.z
    }

    pub const BLACK: Rgb = Rgb::new(0.0, 0.0, 0.0);
}
