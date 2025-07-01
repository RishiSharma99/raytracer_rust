mod hittable;
mod image;
mod image_writer;
mod point;
mod ray;
mod rbg;
mod vec3;

use std::path::Path;

use crate::{
    hittable::{HitRecord, Hittable, Hittables, Sphere},
    image::Image,
    image_writer::{ImageWriter, ppm_writer::PpmFileWriter},
    point::Point3,
    ray::Ray,
    rbg::Rgb,
    vec3::{Vec3, lerp, norm},
};

fn ray_color(ray: &Ray, world: &Hittables) -> Rgb {
    match world.hit(ray, 0.00001..f64::INFINITY) {
        Some(HitRecord { n, .. }) => 0.5 * Rgb::new(n.x + 1.0, n.y + 1.0, n.z + 1.0),
        None => {
            let u = norm(*ray.direction());
            let blend = 0.5 * (u.y + 1.0);

            lerp(&Rgb::new(1.0, 1.0, 1.0), &Rgb::new(0.5, 0.7, 1.0), blend)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16.0 / 9.0 as f64;
    let image_width: usize = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    let mut img = Image::new(image_width, image_height);

    // Create the world
    let mut world = Hittables::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = pixel_origin + ((i as f64) * pixel_delta_u) + ((j as f64) * pixel_delta_v);
            let ray_dir = pixel - camera_center;
            let ray = Ray::new(camera_center, ray_dir);

            let color = ray_color(&ray, &world);

            img[(i, j)] = color;
        }
    }

    let mut writer = PpmFileWriter::new(Path::new("out.ppm"))?;
    writer.write(&img)?;

    Ok(())
}
