use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Context;

use crate::{image::Image, rbg::Rgb};

pub trait ImageWriter {
    fn write(&mut self, image: &Image) -> anyhow::Result<()>;
}

pub struct PpmWriter<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> PpmWriter<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }

    fn write_impl(&mut self, image: &Image) -> anyhow::Result<()> {
        // Writing the header
        let header = format!("P3\n{} {}\n255\n", image.width, image.height);

        self.writer
            .write_all(header.as_bytes())
            .context("An I/O error occurred while writing the PPM header")?;

        let linear_to_gamma = |f: f64| -> f64 { if f > 0.0 { f64::sqrt(f) } else { f } };

        let conv = |f: f64| ((linear_to_gamma(f)).clamp(0.0, 1.0) * 255.99) as u8;
        let serialize_pixel = |px: &Rgb| (conv(px.r()), conv(px.g()), conv(px.b()));

        for p in image.iter() {
            let (r, g, b) = serialize_pixel(p);
            writeln!(&mut self.writer, "{} {} {} ", r, g, b)
                .with_context(|| format!("An I/O error occured while writing pixel data"))?;
        }

        Ok(())
    }

    fn write(&mut self, image: &Image) -> anyhow::Result<()> {
        self.write_impl(image)
            .context("Failed while writing PPM image data")
    }
}

pub struct PpmFileWriter {
    implementation: PpmWriter<BufWriter<File>>,
}

impl PpmFileWriter {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let file_handle = File::create(path).with_context(|| {
            format!(
                "Unable to create or overwrite the output file at : {}",
                path.to_string_lossy()
            )
        })?;

        Ok(Self {
            implementation: PpmWriter::new(BufWriter::new(file_handle)),
        })
    }
}

impl ImageWriter for PpmFileWriter {
    fn write(&mut self, image: &Image) -> anyhow::Result<()> {
        self.implementation.write(image)
    }
}
