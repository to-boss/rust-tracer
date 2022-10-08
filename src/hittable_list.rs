use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anyhing: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|obj| {
            if let Some(hit) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anyhing = Some(hit);
            }
        });

        return hit_anyhing;
    }
}
