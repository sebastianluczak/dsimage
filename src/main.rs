use image::imageops::FilterType;
use image::{DynamicImage};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use rayon::prelude::*;

fn get_filename() -> PathBuf {
    let mut args = env::args();
    args.next();

    if let Some(first_arg) = args.next() {
        PathBuf::from(first_arg)
    } else {
        eprintln!("Usage: <program> <image_file>");
        exit(1);
    }
}

fn create_output_directories() -> Result<(), std::io::Error> {
    fs::create_dir_all("optimized/thumbnails")?;
    Ok(())
}

fn resize_and_save(image: &DynamicImage, target_width: u32, file_path: &Path) -> Result<(), image::ImageError> {
    let aspect_ratio = image.height() as f32 / image.width() as f32;
    let target_height = (target_width as f32 * aspect_ratio) as u32;
    let resized = image.resize(target_width, target_height, FilterType::Lanczos3);
    resized.save(file_path)?;
    Ok(())
}

fn process_image(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(file_path)?;

    let file_stem = file_path.file_stem().unwrap_or_default();
    let output_base = Path::new("optimized").join(file_stem);
    let resized_path = output_base.with_extension("jpg");
    let grayscale_path = output_base.with_file_name(format!("{}_grayscale.jpg", file_stem.to_string_lossy()));

    rayon::join(
        || {
            resize_and_save(&img, 1920, &resized_path).expect("Failed to resize image");
        },
        || {
            let grayscale = img.grayscale();
            resize_and_save(&grayscale, 800, &grayscale_path).expect("Failed to create grayscale image");
        },
    );

    Ok(())
}


fn create_thumbnails(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(file_path)?;
    let file_stem = file_path.file_stem().unwrap_or_default();
    let thumbnail_base = Path::new("optimized/thumbnails").join(file_stem);

    let vector: Vec<u32> = vec![1200, 800, 480];
    vector.par_iter().for_each(|&x| {
        let thumb_path = thumbnail_base.with_file_name(format!("{}_{}.jpg", file_stem.to_string_lossy(), x));
        resize_and_save(&img, x, &thumb_path).expect("Something went wrong!");
    });

    Ok(())
}

fn main() {
    let image_filename = get_filename();

    if let Err(err) = create_output_directories() {
        eprintln!("Error creating output directories: {}", err);
        exit(1);
    }

    rayon::join(
        || {
            if let Err(err) = process_image(&image_filename) {
                eprintln!("Error processing image: {}", err);
                exit(1);
            }
        },
        || {
            if let Err(err) = create_thumbnails(&image_filename) {
                eprintln!("Error creating thumbnails: {}", err);
                exit(1);
            }
        },
    );

    println!("Image processing completed successfully!");
}

