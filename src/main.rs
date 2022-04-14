#![allow(dead_code)]
mod vec3;
mod camera;
mod hittable;
mod ray;
mod shapes;
mod world;

use camera::Camera;
use glam::*;
use image::{Rgb, RgbImage};
use rand::{distributions::Uniform, prelude::Distribution};
use ray::Ray;
use vec3::random_in_unit_sphere;
use world::World;

fn ray_color(r: &Ray, world: &World, depth: u8) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    if let Some(rec) = world.hit(r, 0.0, f32::INFINITY) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}


fn main() {
    let zero_to_one = Uniform::new(0.0f32, 1.0);
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut img = RgbImage::new(width, height);

    // World
    let mut world = World::new();
    world.add_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);
    world.add_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0);

    // Camera
    let camera = Camera::new(aspect_ratio);

    for j in (0..height).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..width {
            let mut total_pixel_color = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + zero_to_one.sample(&mut rng)) / (width - 1) as f32;
                let v = (j as f32 + zero_to_one.sample(&mut rng)) / (height - 1) as f32;

                let r = camera.get_ray(u, v);
                total_pixel_color += ray_color(&r, &world, max_depth);
            }

            img.put_pixel(
                i,
                (height - 1) - j,
                get_pixel_color(total_pixel_color, samples_per_pixel),
            );
        }
    }
    img.save("out.png").unwrap();
}

fn get_pixel_color(v: Vec3, samples: u8) -> Rgb<u8> {
    let scale = 1.0 / samples as f32;
    let v = v * scale;
    Rgb([
        (256.0 * v.x.clamp(0.0, 0.99)) as u8,
        (256.0 * v.y.clamp(0.0, 0.99)) as u8,
        (256.0 * v.z.clamp(0.0, 0.99)) as u8,
    ])
}

