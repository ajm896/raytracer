use crate::geo::*;
use crate::vec3::{Point3, Ray, Vec3};
const IMAGE_WIDTH: usize = 1080;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

#[derive(Default)]
pub struct Camera {
    aspect_ratio: f32,
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}
impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }
    pub fn initialize(&mut self) {
        self.image_width = IMAGE_WIDTH;
        self.aspect_ratio = ASPECT_RATIO;
        self.image_height = IMAGE_HEIGHT;
        let focal_length = 1.;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
        self.center = Point3::default();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
        self.pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&self, world: HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = ray.color(&world);
                pixel_color.write_color();
            }
        }
    }
}
