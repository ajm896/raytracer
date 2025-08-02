use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
const IMAGE_WIDTH: usize = 400;
const ASPECT_RATIO: f32 = 16.0 /9.0;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let focal_length = 1.;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
    let camera_center = Point3::default();

    let viewport_u = Vec3 {
        x:viewport_width, y:0.0, z:0.0
    };
    let viewport_v = Vec3 {
        x:0.0, y:-viewport_height, z:0.0
    };

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left = camera_center - Vec3::new(0.0,0.0,focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray.color();
            pixel_color.write_color();
        }
    }
    //End Render
}

#[derive(Copy, Debug, Default, Clone, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Debug, Default, Clone, PartialEq)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
    fn color(self) -> Color {
        let unit_direction = self.direction.unit_vector();
        let a = 0.5*(unit_direction.y + 1.0);
        (1.0-a)*Color::new(1.,1.,1.) + a*Color::new(0.5,0.7,1.0)


    }
}

type Color = Vec3;
impl Color {
    fn write_color(&self){
        let red_out: i32 = (self.x * 255.0) as i32;
        let green_out: i32 = (self.y * 255.0) as i32;
        let blue_out: i32 = (self.z * 255.0) as i32;
        println!("{red_out} {green_out} {blue_out}");
    }
}

type Point3 = Vec3;

impl Vec3 {
    fn new(x:f32,y:f32,z:f32) -> Self {
        Vec3 { x,  y,  z}
    }
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Vec3{
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
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
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
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
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
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
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
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
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
    fn mul(self, rhs: Vec3) -> Self::Output{
        rhs * self
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
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
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
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
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x,self.y,self.z)
    }

}
