use ndarray::ArrayView3;
use opencv::{prelude::*, videoio, Result};

trait AsArray {
    fn try_as_array(&self) -> Result<ArrayView3<u8>>;
}
impl AsArray for Mat {
    fn try_as_array(&self) -> Result<ArrayView3<u8>> {
        // if !self.is_continuous() {
        //     return Err(anyhow!("Mat is not continuous"));
        // }
        let bytes = self.data_bytes()?;
        let size = self.size()?;
        let a =
            ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes).unwrap();
        Ok(a)
    }
}

pub fn video_to_frames(file_name: String) {
    let mut file = videoio::VideoCapture::from_file(&file_name, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default();
    file.read(&mut frame).unwrap();
    println!("Reading frame {:?}", frame);
    let array = frame.try_as_array().unwrap();
    println!("Reading array {:?}", array);
}
