use crate::geo::*;
use std::rc::Rc;
const IMAGE_WIDTH: usize = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

pub mod geo;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let focal_length = 1.;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
    let camera_center = Point3::default();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut world = HittableList::default();
    let ball = Rc::new(Sphere::new(Vec3::new(0.,0.,-1.), 0.5));
    let ground = Rc::new(Sphere::new(Vec3::new(0.,-100.5,-1.), 100.));

    world.add(ball.clone());
    world.add(ground.clone());

    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray.color(&world);
            pixel_color.write_color();
        }
    }
    //End Render
}
