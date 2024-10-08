use ndarray::{Array3, ArrayView3, OwnedRepr};
use opencv::{prelude::*, videoio, Error, Result};

trait AsArray {
    fn try_as_array(&self) -> ArrayView3<u8>;
}
impl AsArray for Mat {
    fn try_as_array(&self) -> ArrayView3<u8> {
        // if !self.is_continuous() {
        //     return Err(anyhow!("Mat is not continuous"));
        // }
        let bytes = self.data_bytes().unwrap();
        let size = self.size().unwrap();
        let a =
            ArrayView3::from_shape((size.height as usize, size.width as usize, 3), bytes).unwrap();
        a
    }
}

pub fn video_to_frames(file_name: String) -> Array3<u8> {
    let mut file = videoio::VideoCapture::from_file(&file_name, videoio::CAP_ANY).unwrap();
    let mut frame: Mat = Mat::default();
    file.read(&mut frame).unwrap();
    let result = frame.try_as_array().to_owned();
    result
}
