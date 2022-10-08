use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::new_random().unit_vector();

        // Catch degen scatter directions
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::new_random_in_unit_sphere() * self.fuzz,
        );
        if scattered.direction.dot(&rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dialectric {
    pub ir: f32,
}

impl Dialectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * f32::powi(1. - cosine, 5)
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();

        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = f32::min(rec.normal.dot(&-unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let reflectance_bigger =
            Dialectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0);

        let direction = if cannot_refract || reflectance_bigger {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((scattered, attenuation))
    }
}
