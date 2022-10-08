use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let mut normal = (p - self.center) / self.radius;
        let front_face = r.direction.dot(&normal) < 0.0;
        normal = if front_face { normal } else { -normal };

        Some(HitRecord {
            p,
            t,
            normal,
            front_face,
            material: &self.material,
        })
    }
}
