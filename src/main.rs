use std::{f32::INFINITY, io::Result};

use rust_tracer::{
    camera::Camera,
    hittable_list::HittableList,
    img::{Img, Pixel},
    material::{Dialectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::*,
    MAX_DEPTH, SAMPLES_PER_PIXEL,
};

use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;

macro_rules! sphere {
    ($center:ident, $radius:literal, $mat:ident) => {
        Sphere::new($center, $radius, $mat)
    };
    ($point:tt, $radius:literal, $mat:ident) => {
        Sphere::new(Point3::new$point, $radius, $mat)
    };
}

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u32;
    let mut img = Img::new(IMG_HEIGHT, IMG_WIDTH);

    // World Setup
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let v_up = Vec3::new(0., 1., 0.);
    let aperture = 0.1;
    let focus_dist = 10.;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // Render
    let image = (0..IMG_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            println!("\rScanlines remaining: {} ", y);
            (0..IMG_HEIGHT)
                .flat_map(|x| {
                    let col: Color = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let mut rng = thread_rng();
                            let r1: f32 = rng.gen_range(0.0..1.0);
                            let r2: f32 = rng.gen_range(0.0..1.0);
                            let u = (x as f32 + r1) / (IMG_WIDTH as f32 - 1.);
                            let v = (y as f32 + r2) / (IMG_HEIGHT as f32 - 1.);
                            let ray = camera.get_ray(u, v);
                            ray_color(&ray, &world, 0)
                        })
                        .sum();
                    let col = vec![col.x, col.y, col.z];
                    col.iter().map(|c| *c as u8).collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for pixel in image.chunks(3) {
        img.add_pixel(&Color::new(
            pixel[0] as f32,
            pixel[1] as f32,
            pixel[2] as f32,
        ));
    }

    match img.save_file() {
        Ok(()) => println!("Saved file {}!", img.name),
        Err(_) => println!("Error while saving file."),
    };

    Ok(())
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let result: Color;

    if depth > MAX_DEPTH {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth + 1);
        }
        return Color::new(0., 0., 0.);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.);
    result = Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.) * t;
    return result;
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = thread_rng();

    let ground_material = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Box::new(sphere!((0., -1000., 0.), 1000., ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen_range(0.0..1.0);
            let a = a as f32;
            let b = b as f32;
            let center = Vec3::new(
                a + 0.9 * random_f32(&mut rng),
                0.2,
                b + 0.9 * random_f32(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new_random() * Vec3::new_random();
                    let sphere_material = Lambertian { albedo };
                    world.add(Box::new(sphere!(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new_random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.add(Box::new(sphere!(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dialectric { ir: 1.5 };
                    world.add(Box::new(sphere!(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dialectric { ir: 1.5 };
    world.add(Box::new(sphere!((0., 1., 0.), 1., material1)));

    let material2 = Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(sphere!((-4., 1., 0.), 1., material2)));

    let material3 = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Box::new(sphere!((4., 1., 0.), 1., material3)));

    return world;
}

fn random_f32(rng: &mut ThreadRng) -> f32 {
    return rng.gen_range(0.0..1.0);
}
