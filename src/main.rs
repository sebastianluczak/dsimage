use image::imageops::FilterType;
use std::fs;
use std::io;
use std::env;
use std::process::exit;
use image::{DynamicImage};

extern crate image;

fn get_filename() -> String {
    let mut args = env::args();
    args.next();

    if let Some(first_arg) = args.next() {
        first_arg
    } else {
        println!("No arguments were passed!");
        exit(1);
    }
}

fn create_output_directories() -> io::Result<()> {
    fs::create_dir_all("optimized/thumbnails")?;
    println!("Directories created!");
    Ok(())
}

fn process_image(mut file_name: String) -> DynamicImage {
    println!("Opening image {:?}..", file_name);
    let img = image::open(file_name.clone()).unwrap();
    file_name = file_name.replace(".jpg", "").replace(".png", "");
    println!("Height {:?}", img.height());
    println!("Width {:?}", img.width());

    if img.height() > img.width() {
        println!("Detected vertical image...");
        if img.height() >= 2000 {
            println!("Image is too big, resizing to 2000 px in height and maintaining aspect ratio: {:?}", img.height() / 2000);
            img.resize(img.width(), 2000, FilterType::Lanczos3).save(format!("optimized/{file_name}.jpg")).unwrap();
        }
    } else {
        println!("Detected horizontal image...");
        if img.width() >= 2000 {
            println!("Image is too big, resizing to 2000 px in width and height: {:?}", img.width() / 2000);
            img.resize(2000, img.height(), FilterType::Lanczos3).save(format!("optimized/{file_name}.jpg")).unwrap();
        }
    }

    img.grayscale().resize(800, img.height(), FilterType::Lanczos3).save(format!("optimized/{file_name}_grayscale.jpg")).unwrap();
    img
}

fn create_thumbnails(mut file_name: String, image: DynamicImage) {
    println!("Creating thumbnails...");
    file_name = file_name.replace(".jpg", "").replace(".png", "");
    image.resize(1200, image.height(), FilterType::Lanczos3).save(format!("optimized/thumbnails/{file_name}_1200.jpg")).unwrap();
    image.resize(800, image.height(), FilterType::Lanczos3).save(format!("optimized/thumbnails/{file_name}_800.jpg")).unwrap();
    image.resize(480, image.height(), FilterType::Lanczos3).save(format!("optimized/thumbnails/{file_name}_480.jpg")).unwrap();
    println!("Thumbnails created!");
}

fn main() {
    let image_filename = get_filename();
    create_output_directories().expect("Directories could not be created.");
    let processed_image = process_image(image_filename.clone());
    create_thumbnails(image_filename, processed_image);
}
