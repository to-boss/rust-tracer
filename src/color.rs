use crate::{util::clamp, vec3::Color, SAMPLES_PER_PIXEL};

pub fn add_color_to_string(str: &mut String, color: &Color) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1. / (SAMPLES_PER_PIXEL as f32);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let str_to_add = format!(
        "{} {} {}\n",
        (256. * clamp(r, 0., 0.999)) as u64,
        (256. * clamp(g, 0., 0.999)) as u64,
        (256. * clamp(b, 0., 0.999)) as u64,
    );
    str.push_str(str_to_add.as_str());
}
