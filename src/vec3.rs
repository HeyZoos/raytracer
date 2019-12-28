use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        self.sqlen().sqrt()
    }

    pub fn sqlen(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    /// Creates a normalized vector, also known as a unit vector. A normalized
    /// vector has a magnitude of 1.
    ///
    /// # Examples
    ///
    /// Create a unit vector.
    ///
    /// ```
    /// assert_eq!(Vec3(1, 1, 0).norm(), Vec3(0.5, 0.5, 0.0));
    /// ```
    pub fn norm(&self) -> Self {
        let k = 1.0 / self.len();
        Vec3(self.0 * k, self.1 * k, self.2 * k)
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    /// Returns the cross product.
    pub fn cross(self, other: Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    /// Creates a vector with all values initialized to 0.0.
    pub fn zero() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }

    /// Creates a vector with all values initialized to 1.0.
    pub fn one() -> Self {
        Vec3(1.0, 1.0, 1.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3(self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        vector * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn cross() {
        let v1 = Vec3(2.0, 3.0, 4.0);
        let v2 = Vec3(5.0, 6.0, 7.0);
        let result = v1.cross(v2);

        assert_eq!(-3.0, result.x());
        assert_eq!(6.0, result.y());
        assert_eq!(-3.0, result.z());
    }
}
