use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = *center - ray.origin;
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = h.powi(2) - a * c;

    if discriminant < 0. {
        -1.
    } else {
        (h - discriminant.sqrt()) / a
    }
}
#[derive(Copy, Debug, Default, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Debug, Default, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
    pub fn color(self) -> Color {
        let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, &self);

        if t > 0. {
            let n = (self.at(t) - Vec3::new(0., 0., -1.)).unit_vector();
            return 0.5 * Color::new(n.x + 1., n.y + 1., n.z + 1.);
        }

        // Sky Box
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0)
    }
}

pub type Color = Vec3;
impl Color {
    pub fn write_color(&self) {
        let red_out: i32 = (self.x * 255.0) as i32;
        let green_out: i32 = (self.y * 255.0) as i32;
        let blue_out: i32 = (self.z * 255.0) as i32;
        println!("{red_out} {green_out} {blue_out}");
    }
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * self.y,
            y: self.x * rhs.z - self.z * self.x,
            z: self.y * rhs.x - self.x * self.y,
        }
    }
    fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}

trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            let root_pos = (h + sqrtd) / a;
            if root_pos <= ray_tmin || ray_tmax <= root_pos {
                return false;
            }
            hit_record.t = root;
            hit_record.p = ray.at(hit_record.t);
            hit_record.normal = (hit_record.p - self.center) / self.radius;
        }
            true
    }
}
