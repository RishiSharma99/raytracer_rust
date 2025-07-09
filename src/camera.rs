use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::IntoParallelRefMutIterator;

use crate::{
    hittable::Hittable,
    image::Image,
    point::Point3,
    ray::Ray,
    rbg::Rgb,
    vec3::{Vec3, cross, lerp, norm, rand_in_unit_disk},
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

fn ray_color(ray: &Ray, world: &impl Hittable, rng: &mut impl Rng, max_depth: i32) -> Rgb {
    if max_depth <= 0 {
        return Rgb::BLACK;
    }
    match world.hit(ray, 0.00001..f64::INFINITY) {
        Some(h) => match h.mat.scatter(ray, &h, rng) {
            Some((attenuation, new_ray)) => {
                attenuation * ray_color(&new_ray, world, rng, max_depth - 1)
            }
            None => Rgb::BLACK,
        },
        None => {
            let u = norm(*ray.direction());
            let blend = 0.9 * (u.y + 1.0);

            lerp(&Rgb::new(1.0, 1.0, 1.0), &Rgb::new(0.5, 0.7, 1.0), blend)
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, spp: u32, max_depth: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel: spp,
            max_depth,
            vfov: 45.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> Image {
        // Image dimensions
        let image_width = self.image_width;
        let aspect_ratio = self.aspect_ratio;
        let image_height = ((image_width as f64) / aspect_ratio) as usize;

        let camera_center = self.look_from;
        let pixels_samples_scale: f64 = 1.0 / (self.samples_per_pixel as f64);

        // Viewport variables
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));

        // Viewport vectors
        let w = norm(self.look_from - self.look_at);
        let u = norm(cross(self.vup, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            camera_center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_origin = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let mut img = Image::new(image_width, image_height);

        let bar =
            ProgressBar::new((img.width * img.height * (self.samples_per_pixel as usize)) as u64);

        bar.set_style(
            ProgressStyle::with_template( "{spinner:.green} [{elapsed_precise}/{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}% {per_sec}")
                .unwrap(),
        );
        let bar_clone = bar.clone();

        let pixel_slice = img.as_mut_slice();

        pixel_slice
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel_ref)| {
                let i = idx % image_width;
                let j = idx / image_width;

                let mut thread_rng = rand::rng();

                let mut pixel_color = Rgb::new(0.0, 0.0, 0.0);

                for _sp in 0..self.samples_per_pixel {
                    let x_dither: f64 = thread_rng.random::<f64>();
                    let y_dither: f64 = thread_rng.random::<f64>();

                    let pixel_sample = pixel_origin
                        + (((i as f64) + x_dither) * pixel_delta_u)
                        + (((j as f64) + y_dither) * pixel_delta_v);

                    let ray_origin = if self.defocus_angle <= 0.0 {
                        camera_center
                    } else {
                        let p = rand_in_unit_disk(&mut thread_rng);
                        camera_center + (p.x * defocus_disk_u) + (p.y * defocus_disk_v)
                    };

                    let ray_dir = pixel_sample - ray_origin;
                    let ray = Ray::new(ray_origin, ray_dir);

                    pixel_color =
                        pixel_color + ray_color(&ray, world, &mut thread_rng, self.max_depth);
                    bar_clone.inc(1);
                }
                *pixel_ref = pixel_color * pixels_samples_scale;
            });

        bar.finish();
        img
    }
}
