use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::geo::*;
use std::rc::Rc;

pub mod camera;
pub mod geo;

fn main() {
    let mut world = HittableList::default();
    let ball = Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5));
    let ground = Rc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.));

    world.add(ball.clone());
    world.add(ground.clone());

    let mut camera = Camera::new();
    camera.initialize();
    camera.render(world);
}
