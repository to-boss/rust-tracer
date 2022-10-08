use std::{
    f32::INFINITY,
    fs::File,
    io::{Result, Write},
};

use rust_tracer::{
    camera::Camera, color::*, hittable_list::HittableList, ray::Ray, sphere::Sphere, vec3::*,
    MAX_DEPTH, SAMPLES_PER_PIXEL,
};

use rand::{thread_rng, Rng};

macro_rules! sphere {
    // input:  ((0., 0., -1.), 0.5);
    // output: Sphere::new(Vec3::new(0., 0., -1.), 0.5)
    ($point:tt, $radius:literal) => {
        Sphere::new(Point3::new$point, $radius)
    };
}

fn main() -> Result<()> {
    let mut rng = thread_rng();

    // Image
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u32;

    // World Setup
    let mut world = HittableList::new();
    let sphere = sphere!((0., 0., -1.), 0.5);
    let sphere2 = sphere!((0., -100.5, -1.), 100.);
    world.add(Box::new(sphere));
    world.add(Box::new(sphere2));

    // Camera
    let camera = Camera::new(ASPECT_RATIO);

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
    let mut rng = thread_rng();

    if depth > MAX_DEPTH {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        let target = Vec3::random_in_hemisphere(&mut rng, &hit.normal);
        let new_color = ray_color(&Ray::new(hit.p, target - hit.p), world, depth + 1);
        result = Color::new(new_color.x, new_color.y, new_color.z) * 0.5;
        return result;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.);
    result = Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.) * t;
    return result;
}
