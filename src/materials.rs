use glam::Vec3;

use crate::{ray::Ray, hittable::HitRecord, vec3::Vec3Helpers};

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub struct MaterialHandle {
    material_type: Material,
}

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 },
}

impl MaterialHandle {
    pub fn new(material_type: Material) -> Self {
        Self { material_type }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {

        match self.material_type {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
        
                Some(Scatter {
                    attenuation: albedo,
                    ray: Ray::new(rec.p, scatter_direction),
                })
            },
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(r_in.direction.normalize(), rec.normal);
                let ray = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
                
                if ray.direction.dot(rec.normal) > 0.0 {
                    Some(Scatter {
                        attenuation: albedo,
                        ray,
                    })
                } else {
                    None
                }
                
            },
            Material::Dielectric { ir } => {
                let refracted_ratio = if rec.front_face {
                    1.0 / ir
                } else {
                    ir
                };
        
                let unit_direction = r_in.direction.normalize();
                let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).abs().sqrt();
        
                let cannot_refract = refracted_ratio * sin_theta > 1.0;
                let direction = if cannot_refract {
                    reflect(unit_direction, rec.normal)
                } else {
                    refract(unit_direction, rec.normal, refracted_ratio)
                };

                Some(Scatter {
                    attenuation: Vec3::ONE,
                    ray: Ray::new(rec.p, direction),
                })
            },
        }
    }
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}