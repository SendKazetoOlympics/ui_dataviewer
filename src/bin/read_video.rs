use clap::Parser;
use ui_dataviewer::video_reader::video_to_frames;

#[derive(Parser)]
struct Cli {
    file_name: String,
}

fn main() {
    let args = Cli::parse();

    println!("Reading file with name {:?}", args.file_name);
    video_to_frames(args.file_name);
}
