use glam::{Vec3, vec3};
use rand::Rng;

pub trait Vec3Helpers {
    fn random() -> Self;
    fn random_range(min: f32, max: f32) -> Self;
    fn near_zero(&self) -> bool;
    fn random_in_unit_sphere() -> Vec3;
    fn random_in_hemisphere(normal: Vec3) -> Vec3;
    fn random_unit_vector() -> Vec3;
    fn random_in_unit_disk() -> Vec3;

}


impl Vec3Helpers for Vec3 {
    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s)
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        vec3(rng.gen_range(-1.0..1.0),
         rng.gen_range(-1.0..1.0),
         rng.gen_range(-1.0..1.0))
    }

    fn random_range(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        vec3(rng.gen_range(min..max),
         rng.gen_range(min..max),
         rng.gen_range(min..max))
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        // In the same hemisphere as the normal
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut rng = rand::thread_rng();
            let p = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}