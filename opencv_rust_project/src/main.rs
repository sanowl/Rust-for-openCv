extern crate opencv;

use opencv::core::Mat;
use opencv::highgui;

fn main() {
    // Initialize OpenCV
    opencv::highgui::named_window("Image Window", opencv::highgui::WINDOW_NORMAL);

    // Load an image
    let img = Mat::from_slice_2d(&vec![vec![0; 100]; 100]).unwrap(); // Replace with your image path

    // Display the image
    opencv::highgui::imshow("Image Window", &img).unwrap();

    // Wait for a key press
    opencv::highgui::wait_key(0).unwrap();
}
