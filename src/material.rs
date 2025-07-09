use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    rbg::Rgb,
    vec3::{Vec3, dot, norm, rand_unit_vec},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Rgb },
    Metal { albedo: Rgb, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Rgb, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_direction = hit_record.n + rand_unit_vec(rng);

                if scatter_direction.near_zero() {
                    Some((*albedo, Ray::new(hit_record.p, hit_record.n)))
                } else {
                    Some((*albedo, Ray::new(hit_record.p, scatter_direction)))
                }
            }

            Self::Metal { albedo, fuzz } => {
                let fuzz: f64 = fuzz.clamp(0.0, 1.0);

                let reflected =
                    norm(reflect(*ray.direction(), hit_record.n)) + (fuzz * rand_unit_vec(rng));

                let scattered = Ray::new(hit_record.p, reflected);
                if dot(*scattered.direction(), hit_record.n) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }

            Self::Dielectric { refraction_index } => {
                let attenuation = Rgb::new(1.0, 1.0, 1.0);
                let ri = if hit_record.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_dir = norm(*ray.direction());

                let cos_theta = f64::min(dot(-unit_dir, hit_record.n), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                let direction = if ri * sin_theta > 1.0 || reflactance(cos_theta, ri) > rng.random()
                {
                    reflect(unit_dir, hit_record.n)
                } else {
                    refract(unit_dir, hit_record.n, ri)
                };

                Some((attenuation, Ray::new(hit_record.p, direction)))
            }
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - (2.0 * dot(v, normal) * normal)
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, normal), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.len_sqrd())) * normal;

    r_out_perp + r_out_parallel
}

fn reflactance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_2 = r0 * r0;

    r0_2 + (1.0 - r0_2) * f64::powi(1.0 - cosine, 5)
}
