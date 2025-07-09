mod camera;
mod hittable;
mod image;
mod image_writer;
mod material;
mod point;
mod ray;
mod rbg;
mod vec3;

use std::path::Path;

use rand::{Rng, rng};

use crate::{
    camera::Camera,
    hittable::{Hittables, Sphere},
    image_writer::{ImageWriter, PpmFileWriter},
    material::Material,
    point::Point3,
    rbg::Rgb,
    vec3::Vec3,
};

fn random_world() -> Hittables {
    let mut world = Hittables::new();

    let mut rng = rng();

    //ground
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Rgb::new(0.5, 0.5, 0.5),
        },
    ));

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice = rng.random::<f64>();

            let center = Vec3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if mat_choice < 0.8 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            albedo: Rgb::rand(&mut rng) * Rgb::rand(&mut rng),
                        },
                    ))
                } else if mat_choice < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Rgb::rand_in_range(&mut rng, 0.5, 1.0),
                            fuzz: rng.random_range(0.0..0.5),
                        },
                    ))
                } else {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric {
                            refraction_index: 1.5,
                        },
                    ));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric {
            refraction_index: 1.5,
        },
    ));

    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Rgb::new(0.4, 0.2, 0.1),
        },
    ));

    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Rgb::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));

    world
}

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16.0 / 9.0 as f64;
    let image_width: usize = 1200;

    let mut camera = Camera::new(aspect_ratio, image_width, 500, 50);
    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let world = random_world();

    let img = camera.render(&world);

    let mut writer = PpmFileWriter::new(Path::new("out.ppm"))?;
    writer.write(&img)?;

    Ok(())
}
