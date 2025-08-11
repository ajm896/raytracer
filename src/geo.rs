use crate::vec3::{Point3, Ray, Vec3};
use core::f32;
use std::rc::Rc;
pub mod vec3;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (f32::consts::PI / 180.0)
}

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
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

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        true
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_hr = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_hr) {
                hit_anything = true;
                closest_so_far = temp_hr.t;
                *hit_record = temp_hr;
            }
        }
        hit_anything
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degrees_are_converted_to_radians() {
        let pi = f32::consts::PI;
        assert!((degrees_to_radians(0.0) - 0.0).abs() < 1e-6);
        assert!((degrees_to_radians(90.0) - pi / 2.0).abs() < 1e-6);
        assert!((degrees_to_radians(180.0) - pi).abs() < 1e-6);
        assert!((degrees_to_radians(360.0) - 2.0 * pi).abs() < 1e-6);
    }
}
