use glam::{Vec3, vec3};

use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        
        let viewport_height = 2.0f32;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0f32;

        let origin = vec3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, viewport_height, 0.0);

        Self {
            origin,
            vertical,
            horizontal,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}