use crate::camera::Camera;
use crate::geo::*;
use crate::vec3::Point3;
use std::rc::Rc;

pub mod camera;
pub mod geo;

fn main() {
    let mut world = HittableList::default();
    let ball1 = Rc::new(Sphere::new(Point3::new(-0.5, 0., -1.), 0.5));
    let ball2 = Rc::new(Sphere::new(Point3::new(0.5, 0., -1.), 0.5));
    let ground = Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    world.add(ball1.clone());
    world.add(ball2.clone());
    world.add(ground.clone());

    let mut camera = Camera::new();
    camera.initialize();
    camera.render(world);
}
