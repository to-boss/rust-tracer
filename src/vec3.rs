use std::ops::*;

use rand::{thread_rng, Rng};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_random() -> Self {
        let mut rand = thread_rng();
        let x: f32 = rand.gen_range(0.0..1.0);
        let y: f32 = rand.gen_range(0.0..1.0);
        let z: f32 = rand.gen_range(0.0..1.0);

        Vec3 { x, y, z }
    }

    pub fn new_random_range(range: Range<f32>) -> Vec3 {
        let mut rand = thread_rng();
        let x: f32 = rand.gen_range(range.clone());
        let y: f32 = rand.gen_range(range.clone());
        let z: f32 = rand.gen_range(range);

        Vec3 { x, y, z }
    }

    pub fn new_random_in_unit_sphere() -> Vec3 {
        let mut in_sphere = false;
        let mut p: Vec3 = Vec3::new_random_range(-1.0..1.0);

        while in_sphere == false {
            p = Vec3::new_random_range(-1.0..1.0);

            if p.length_squared() >= 1.0 {
                continue;
            } else {
                in_sphere = true;
            }
        }
        return p;
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::new_random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut in_unit = false;
        let mut p: Vec3 = Vec3::new_random_range(-1.0..1.0);
        p.z = 0.;

        while in_unit == false {
            p = Vec3::new_random_range(-1.0..1.0);

            if p.length_squared() >= 1.0 {
                continue;
            } else {
                in_unit = true;
            }
        }
        return p;
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(-self.dot(n), 1.0);
        let r_out_perp = (*self + (*n) * cos_theta) * etai_over_etat;
        let r_out_parallel = (*n) * -f32::abs(1.0 - r_out_perp.length_squared()).sqrt();
        r_out_parallel + r_out_perp
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (*normal) * self.dot(normal) * 2.
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }
}

// Operator Overloading
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[test]
fn test_add() {
    let vec1 = Vec3::new(1.0, 1.0, 1.0);
    let vec2 = Vec3::new(1.0, 1.0, 1.0);
    assert_eq!(vec1 + vec2, Vec3::new(2.0, 2.0, 2.0));
}

#[test]
fn test_sub() {
    let vec1 = Vec3::new(1.0, 1.0, 1.0);
    let vec2 = Vec3::new(1.0, 1.0, 1.0);
    assert_eq!(vec1 - vec2, Vec3::new(0.0, 0.0, 0.0));
}

#[test]
fn test_mul() {
    let vec1 = Vec3::new(1.0, 1.0, 1.0);
    let vec2 = Vec3::new(2.0, 2.0, 2.0);
    let num: f32 = 5.0;
    assert_eq!(vec1 * vec2, Vec3::new(2.0, 2.0, 2.0));
    assert_eq!(vec1 * num, Vec3::new(5.0, 5.0, 5.0));
}

#[test]
fn test_div() {
    let vec1 = Vec3::new(2.0, 2.0, 2.0);
    let vec2 = Vec3::new(12.0, 12.0, 12.0);
    let num: f32 = 5.0;
    assert_eq!(vec2 / vec1, Vec3::new(6.0, 6.0, 6.0));
    assert_eq!(vec1 / num, Vec3::new(0.4, 0.4, 0.4));
}
