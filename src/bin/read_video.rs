use std::borrow::Borrow;

use clap::Parser;
use ndarray::{s, Array, ShapeBuilder};
use opencv::core::MatTraitConstManual;
use rerun::TensorData;
use ui_dataviewer::video_reader::video_to_frames;

#[derive(Parser)]
struct Cli {
    file_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let rec = rerun::RecordingStreamBuilder::new("SKO_inspector").spawn()?;
    println!("Reading file with name {:?}", args.file_name);
    let array: TensorData = video_to_frames(args.file_name)
        .as_slice()
        .unwrap()
        .try_into()
        .unwrap();
    rec.log(
        "image",
        &rerun::Image::from_color_model_and_tensor(rerun::ColorModel::RGB, array)?,
    )?;

    Ok(())
}
