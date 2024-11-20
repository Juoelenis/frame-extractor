use std::process::{Command, exit};
use std::fs;
use std::path::Path;

fn extract_frames(video_path: &str, output_dir: &str) {
    // Ensure output directory exists
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    // Command to extract frames using ffmpeg at 1 second intervals
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path) // Path to the video
        .arg("-vf")
        .arg("fps=1") // Extract 1 frame per second
        .arg("-q:v")
        .arg("2") // Set the quality (lower value = higher quality)
        .arg(format!("{}/frame%04d.jpg", output_dir)) // Output directory and file naming format
        .status()
        .expect("Failed to execute ffmpeg");

    if !status.success() {
        eprintln!("Error: ffmpeg command failed.");
        exit(1);
    }

    println!("Frames extracted successfully to {}", output_dir);
}

fn main() {
    let video_path = "path/to/your/video.mp4"; // Path to your video file
    let output_dir = "frames"; // Directory to store the frames

    extract_frames(video_path, output_dir);
}
