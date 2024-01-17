extern crate opencv;

use opencv::{
    core::{Mat, Point, Scalar, Size},
    highgui, imgcodecs, imgproc, objdetect,
    prelude::*,
    videoio,
};

fn main() -> opencv::Result<()> {
    // Initialize OpenCV
    highgui::named_window("Video Feed", highgui::WINDOW_AUTOSIZE)?;

    // Load a face cascade classifier
    let mut face_cascade =
        objdetect::CascadeClassifier::new("path/to/haarcascade_frontalface_default.xml")?;

    // Capture video from the default camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        println!("Error: Camera could not be opened");
        return Ok(());
    }

    loop {
        let mut frame = Mat::default()?;
        if !cam.read(&mut frame)? {
            break;
        }

        if frame.size()?.width > 0 {
            // Convert to grayscale for face detection
            let mut gray = Mat::default()?;
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

            // Face detection
            let mut faces = opencv::types::VectorOfRect::new();
            face_cascade.detect_multi_scale(
                &gray,
                &mut faces,
                1.1,
                2,
                0,
                Size::new(30, 30),
                Size::new(500, 500),
            )?;

            // Draw rectangles around detected faces
            for face in faces.iter() {
                imgproc::rectangle(
                    &mut frame,
                    face,
                    Scalar::new(0.0, 255.0, 0.0, 0.0), // Green color
                    2,
                    imgproc::LINE_8,
                    0,
                )?;
            }

            // Display the video feed with detected faces
            highgui::imshow("Video Feed", &frame)?;
        }

        // Break the loop when 'ESC' key is pressed
        if highgui::wait_key(10)? == 27 {
            break;
        }
    }

    Ok(())
}
