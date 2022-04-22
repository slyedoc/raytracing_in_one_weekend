use glam::Vec3;

use crate::{shapes::Sphere, ray::Ray, hittable::{HitRecord, HitTable}, materials::{MaterialHandle, Material}};

pub struct World {
    spheres: Vec<Sphere>,
    materials: Vec<MaterialHandle>,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, p: Vec3, r: f32, material: usize) {
        self.spheres.push(Sphere::new(p,r, material));
    }

    pub fn add_material(&mut self, t: Material) -> usize {
        self.materials.push(MaterialHandle::new(t));
        self.materials.len() - 1
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

    pub fn get_material(&self, id: usize) -> &MaterialHandle {
        &self.materials[id]
    }
}