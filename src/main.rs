use std::{
    f32::INFINITY,
    fs::File,
    io::{Result, Write},
};

use rust_tracer::{color::*, hittable_list::HittableList, ray::Ray, sphere::Sphere, vec3::*};

fn main() -> Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u32;

    // World
    let mut world = HittableList::new();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere));
    world.add(Box::new(sphere2));

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizantal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizantal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    let mut img_file = File::create("image.ppm")?;
    let mut file_content = String::new();
    file_content.push_str(format!("P3\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT).as_str());

    for j in (0..IMG_HEIGHT).rev() {
        println!("\rScanlines remaining: {} ", j);
        for i in 0..IMG_WIDTH {
            let u = i as f32 / (IMG_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMG_HEIGHT as f32 - 1.0);
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + horizantal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&r, &world);
            add_color_to_string(&mut file_content, &pixel_color);
        }
    }

    img_file.write_all(file_content.as_bytes())?;
    println!("Done.");
    Ok(())
}

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discrimant = half_b * half_b - a * c;

    if discrimant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discrimant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let result: Color;

    if world.hit(r, 0.0, INFINITY) {
        result = (Color::new(1.0, 1.0, 1.0)) * 0.5;
        return result;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    result = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    result
}
