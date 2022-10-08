use crate::vec3::Color;

pub fn add_color_to_string(str: &mut String, color: &Color) {
    let str_to_add = format!(
        "{} {} {}\n",
        (255.999 * color.x) as u64,
        (255.999 * color.y) as u64,
        (255.999 * color.z) as u64,
    );
    str.push_str(str_to_add.as_str());
}
