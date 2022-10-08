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

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new_empty();
        let mut hit_anyhing = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|obj| {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anyhing = true;
                closest_so_far = temp_rec.t;
            }
        });

        return hit_anyhing;
    }
}
