use std::{
    f32::INFINITY,
    fs::File,
    io::{Result, Write},
};

use rust_tracer::{
    camera::Camera,
    color::*,
    hittable_list::HittableList,
    material::{Dialectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::*,
    MAX_DEPTH, SAMPLES_PER_PIXEL,
};

use rand::{rngs::ThreadRng, thread_rng, Rng};

macro_rules! sphere {
    ($center:ident, $radius:literal, $mat:ident) => {
        Sphere::new($center, $radius, $mat)
    };
    ($point:tt, $radius:literal, $mat:ident) => {
        Sphere::new(Point3::new$point, $radius, $mat)
    };
}

fn main() -> Result<()> {
    let mut rng = thread_rng();

    // Image
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u32;

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
    let mut img_file = File::create("image.ppm")?;
    let mut file_content = String::new();
    file_content.push_str(format!("P3\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT).as_str());

    for j in (0..IMG_HEIGHT).rev() {
        println!("\rScanlines remaining: {} ", j);
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let r1: f32 = rng.gen_range(0.0..1.0);
                let r2: f32 = rng.gen_range(0.0..1.0);
                let u = (i as f32 + r1) / (IMG_WIDTH as f32 - 1.);
                let v = (j as f32 + r2) / (IMG_HEIGHT as f32 - 1.);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, 0);
            }
            add_color_to_string(&mut file_content, &pixel_color);
        }
    }

    img_file.write_all(file_content.as_bytes())?;
    println!("Done.");
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
