use std::fs;

pub fn before_runtime() {
    let build_dir = "build";
    let images_dir = "build/images";
    let json_dir = "build/json";

    // Check if the build directory exists
    if !fs::metadata(build_dir).is_ok() {
        // Create the build directory
        fs::create_dir(build_dir).unwrap();

        // Create the images and json subdirectories
        fs::create_dir(images_dir).unwrap();
        fs::create_dir(json_dir).unwrap();

        println!("Created build directory with images and json subdirectories");
    } else {
        println!("Build directory already exists");
    }
}
