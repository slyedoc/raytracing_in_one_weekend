use glam::{Vec3, vec3};
use rand::Rng;

pub trait Random {
    fn random() -> Self;
    fn random_range(min: f32, max: f32) -> Self;
}

impl Random for Vec3 {
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
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random();
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}