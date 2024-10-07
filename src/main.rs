use image::DynamicImage;
use rerun::{demo_util::grid, external::glam};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("SKO_inspector").spawn()?;

    // rec.log("frame", &rerun::Image::from_dynamic_image(image))?;

    Ok(())
}
