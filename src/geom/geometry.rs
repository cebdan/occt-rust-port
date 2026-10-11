use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn distance_to(self, other: Self) -> f64 {
        (self - other).length()
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub rows: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn identity() -> Self {
        Self {
            rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn rotation_x(angle_rad: f64) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Self {
            rows: [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        }
    }

    pub fn rotation_y(angle_rad: f64) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Self {
            rows: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        }
    }

    pub fn rotation_z(angle_rad: f64) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        Self {
            rows: [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn transform_point(&self, p: Point3) -> Point3 {
        Point3::new(
            self.rows[0][0] * p.x + self.rows[0][1] * p.y + self.rows[0][2] * p.z,
            self.rows[1][0] * p.x + self.rows[1][1] * p.y + self.rows[1][2] * p.z,
            self.rows[2][0] * p.x + self.rows[2][1] * p.y + self.rows[2][2] * p.z,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3 {
    pub matrix: Matrix3,
    pub translation: Vector3,
}

impl Transform3 {
    pub fn identity() -> Self {
        Self {
            matrix: Matrix3::identity(),
            translation: Vector3::zero(),
        }
    }

    pub fn translate(dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            matrix: Matrix3::identity(),
            translation: Vector3::new(dx, dy, dz),
        }
    }

    pub fn rotate_x(angle_rad: f64) -> Self {
        Self {
            matrix: Matrix3::rotation_x(angle_rad),
            translation: Vector3::zero(),
        }
    }

    pub fn transform_point(&self, p: Point3) -> Point3 {
        let rotated = self.matrix.transform_point(p);
        rotated + self.translation
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,
}

impl Plane {
    pub fn new(origin: Point3, normal: Vector3) -> Self {
        Self {
            origin,
            normal: normal.normalized(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box3 {
    pub min: Point3,
    pub max: Point3,
}

impl Box3 {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn dimensions(&self) -> Vector3 {
        self.max - self.min
    }

    pub fn volume(&self) -> f64 {
        let d = self.dimensions();
        d.x * d.y * d.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere3 {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere3 {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn volume(&self) -> f64 {
        4.0 / 3.0 * std::f64::consts::PI * self.radius.powi(3)
    }
}

#[cfg(test)]
mod tests {
    use super::{Box3, Matrix3, Plane, Point3, Sphere3, Transform3, Vector3};

    #[test]
    fn point_distance_is_euclidean() {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(3.0, 4.0, 0.0);
        assert!((a.distance_to(b) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn vector_dot_and_cross_are_correct() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        assert!((a.dot(b) - 32.0).abs() < 1e-9);
        let c = a.cross(b);
        assert!((c.x + 3.0).abs() < 1e-9);
        assert!((c.y - 6.0).abs() < 1e-9);
        assert!((c.z + 3.0).abs() < 1e-9);
    }

    #[test]
    fn matrix_rotation_x_keeps_length() {
        let p = Point3::new(0.0, 1.0, 0.0);
        let rotated = Matrix3::rotation_x(std::f64::consts::PI / 2.0).transform_point(p);
        assert!((rotated.x).abs() < 1e-9);
        assert!((rotated.y).abs() < 1e-9);
        assert!((rotated.z - 1.0).abs() < 1e-9);
    }

    #[test]
    fn transform_translate_moves_point() {
        let t = Transform3::translate(1.0, 2.0, 3.0);
        let p = Point3::origin();
        let moved = t.transform_point(p);
        assert_eq!(moved, Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn plane_normal_is_normalized() {
        let p = Plane::new(Point3::origin(), Vector3::new(3.0, 0.0, 0.0));
        assert!((p.normal.length() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn box_volume_is_correct() {
        let b = Box3::new(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 3.0, 4.0));
        assert!((b.volume() - 24.0).abs() < 1e-9);
    }

    #[test]
    fn sphere_volume_is_correct() {
        let s = Sphere3::new(Point3::origin(), 1.0);
        let expected = 4.0 / 3.0 * std::f64::consts::PI;
        assert!((s.volume() - expected).abs() < 1e-6);
    }
}

