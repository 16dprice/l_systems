use std::fs::File;
use std::io::BufWriter;
use png::{Encoder, Compression, FilterType, ColorType};

pub fn write_png(image_data: &[u8], width: u16, height: u16, output_path: &str) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let ref mut writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, width as u32, height as u32);

    encoder.set_compression(Compression::Fast);
    encoder.set_filter(FilterType::NoFilter);
    encoder.set_color(ColorType::Rgba);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(image_data)?;

    Ok(())
}
