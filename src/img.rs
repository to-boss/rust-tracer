use std::{fmt::Display, fs::File, io::Write, ops::Range};

use crate::{
    util::clamp,
    vec3::{Color, Vec3},
    SAMPLES_PER_PIXEL,
};

pub struct Img {
    pub height: u32,
    pub width: u32,
    pub name: String,
    pub pixels: Vec<Pixel>,
    pub content: String,
    pub file: File,
}

impl Img {
    pub fn new(height: u32, width: u32) -> Img {
        let name = "image.ppm".to_string();
        let file = File::create(&name).unwrap();
        let content = format!("P3\n{} {} \n255\n", width, height);
        let pixels = Vec::new();

        Img {
            height,
            width,
            name,
            pixels,
            content,
            file,
        }
    }

    pub fn save_file(&mut self) -> Result<(), std::io::Error> {
        self.file.write_all(self.content.as_bytes())
    }

    pub fn add_pixel(&mut self, color: &Color) {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        let scale = 1. / (SAMPLES_PER_PIXEL as f32);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let pixel = Pixel { r, g, b };

        self.content.push_str(&pixel.to_string());
        self.pixels.push(pixel);
    }

    pub fn iter_heigth(&self) -> Range<u32> {
        (0..self.height).into_iter()
    }

    pub fn iter_width(&self) -> Range<u32> {
        (0..self.width).into_iter()
    }
}

pub struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

impl Pixel {
    fn from_vec3(vec: &Vec3) -> Pixel {
        Pixel {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{} {} {}\n",
            (256. * clamp(self.r, 0., 0.999)) as u64,
            (256. * clamp(self.g, 0., 0.999)) as u64,
            (256. * clamp(self.b, 0., 0.999)) as u64,
        );

        write!(f, "{}", str)
    }
}
