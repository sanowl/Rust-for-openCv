extern crate opencv;

use opencv::{
    core::{Mat, Size, CV_8UC3},
    highgui, imgcodecs, imgproc,
    prelude::*,
    videoio,
};

fn main() -> opencv::Result<()> {
    // Initialize OpenCV
    highgui::named_window("Image Window", highgui::WINDOW_NORMAL)?;

    // Load an image
    // Replace 'image_path.jpg' with the path to your image
    let mut img = imgcodecs::imread("image_path.jpg", imgcodecs::IMREAD_COLOR)?;

    // Check if the image is loaded
    if img.empty() {
        println!("Could not open or find the image");
        return Ok(());
    }

    // Convert to grayscale
    let mut gray = Mat::default()?;
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply Gaussian blur
    imgproc::gaussian_blur(
        &gray,
        &mut gray,
        Size::new(7, 7),
        0.0,
        0.0,
        imgproc::BORDER_DEFAULT,
    )?;

    // Capture video from the default camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    // Check if the camera opened successfully
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
            highgui::imshow("Camera", &frame)?;
        }

        // Display the processed image
        highgui::imshow("Processed Image", &gray)?;

        // Break the loop when 'ESC' key is pressed
        if highgui::wait_key(10)? == 27 {
            break;
        }
    }

    Ok(())
}
