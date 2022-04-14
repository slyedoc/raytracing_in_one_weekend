use glam::Vec3;

use crate::{shapes::sphere::Sphere, ray::Ray, hittable::{HitRecord, HitTable}};

pub struct World {
    spheres: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, p: Vec3, r: f32) {
        self.spheres.push(Sphere::new(p,r,));
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_distance = t_max;

        for sphere in &self.spheres {
            if let Some(hit) = sphere.hit(r, t_min, closest_distance) {
                closest_distance = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}