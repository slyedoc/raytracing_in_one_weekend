#![allow(dead_code)]
mod camera;
mod hittable;
mod materials;
mod ray;
mod scene;
mod shapes;
mod vec3;
mod world;

use camera::Camera;
use glam::*;
use image::{Rgb, RgbImage};
use rand::{Rng};
use ray::Ray;
use world::World;
use rayon::prelude::*;

fn ray_color(r: &Ray, world: &World, depth: u8) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        if let Some(scatter) = world
            .get_material(rec.material)
            .scatter(r, &rec) {
                return scatter.attenuation * ray_color(&scatter.ray, world, depth - 1);
            }
        return Vec3::ZERO;
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let width = 1024;
    let height = (width as f32 / aspect_ratio) as u32;

    // Quality
    let samples_per_pixel = 500usize;
    let max_depth = 20;

    // Camera
    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.01,
        10.0, // (look_from - look_at).length();
    );

    // World
    let mut world = World::new();


    //scene::fov_scene(&mut world);
    //scene::three_ball_scene(&mut world);
    scene::book_cover_scene(&mut world);

    
    let mut img = RgbImage::new(width, height);

    for j in (0..height).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..width {
            
            let color = (0..samples_per_pixel)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / width as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / height as f32;
                let r = camera.get_ray(u, v);
                ray_color(&r, &world, max_depth)
            })
            .collect::<Vec<Vec3>>()
            .iter()
            .sum();

            img.put_pixel(
                i,
                (height - 1) - j,
                correct_pixel_color(color, samples_per_pixel),
            );
        }
    }
    img.save("out.png").unwrap();
}

fn correct_pixel_color(v: Vec3, samples: usize) -> Rgb<u8> {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples as f32;
    let r = (scale * v.x).sqrt();
    let g = (scale * v.y).sqrt();
    let b = (scale * v.z).sqrt();

    Rgb([
        (256.0 * r.clamp(0.0, 0.99)) as u8,
        (256.0 * g.clamp(0.0, 0.99)) as u8,
        (256.0 * b.clamp(0.0, 0.99)) as u8,
    ])
}
