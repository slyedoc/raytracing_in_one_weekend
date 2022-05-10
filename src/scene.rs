use std::f32::consts::PI;

use glam::{Vec3, vec3};
use rand::Rng;

use crate::{world::World, material::{ Material}, vec3::Vec3Helpers};

pub fn book_cover_scene(world: &mut World) {

    let mut rng = rand::thread_rng();

    let material_ground = world.add_material(Material::Lambertian { albedo: vec3(0.5, 0.5, 0.5) });
    world.add_sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0f32..1.0);
            let center = vec3(a as f32 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f32 + 0.9 * rng.gen_range(0.0..1.0));

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random_range(0.5, 1.0);
                    let mat = world.add_material(Material::Lambertian { albedo });
                    world.add_sphere(center, 0.2, mat);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0f32..0.5);
                    let mat = world.add_material(Material::Metal { albedo, fuzz });
                    world.add_sphere(center, 0.2, mat);
                } else {
                    let mat = world.add_material(
                        Material::Dielectric { ir: 1.5 },
                    );
                    world.add_sphere(center, 0.2, mat);
                }
            }
        }
    }
    let material1 = world.add_material(Material::Dielectric { ir: 1.5 });
    world.add_sphere(Vec3::new(0.0, 1.0, 0.0), 1.0, material1);

    let material2 = world.add_material(Material::Lambertian { albedo: vec3(0.4, 0.2, 0.1) });
    world.add_sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2);

    let material3 = world.add_material(Material::Metal{ albedo: vec3(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add_sphere(Vec3::new(4.0, 1.0, 0.0), 1.0, material3);
}

pub fn fov_scene(world: &mut World) {
    // Materials
    let r = PI / 4.0;
    let material_left = world.add_material(Material::Lambertian { albedo: vec3(0.0, 0.0, 1.0) });
    let material_right = world.add_material(Material::Lambertian{ albedo: vec3(1.0, 0.0, 0.0) });
    world.add_sphere(Vec3::new(-r, 0.0, -1.0), r, material_left);
    world.add_sphere(Vec3::new(r, 0.0, -1.0), r, material_right);
}

pub fn three_ball_scene(world: &mut World) {
    let material_ground = world.add_material(Material::Lambertian{ albedo: vec3(0.8, 0.8, 0.0) });
    let material_center = world.add_material(Material::Lambertian{ albedo: vec3(0.1, 0.2, 0.5)});
    let material_left   = world.add_material(Material::Dielectric{ ir: 1.5 });
    let material_right  = world.add_material(Material::Metal { albedo: vec3(0.8, 0.6, 0.2), fuzz: 0.0 });
    // Objects
    world.add_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    world.add_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    world.add_sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    world.add_sphere(Vec3::new(-1.0, 0.0, -1.0), -0.45, material_left);
    world.add_sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
}
