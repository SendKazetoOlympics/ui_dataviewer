use opencv::{highgui, prelude::*, videoio, Result};

pub fn video_to_frames(file_name: String) {
    let mut file = videoio::VideoCapture::from_file(&file_name, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default();
    file.read(&mut frame).unwrap();
    println!("Reading frame {:?}", frame);
}
