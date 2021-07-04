use image::{self, DynamicImage, GenericImageView, Pixel, Rgb};
use std::env;

const ASCII_CHARACTERS: &str =
    "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn read_pixel_data(img: &DynamicImage) -> Vec<Rgb<u8>> {
    img.pixels().map(|f| f.2.to_rgb()).collect()
}

fn map_rgb_to_brightness(img_data: &[Rgb<u8>]) -> Vec<f32> {
    img_data
        .iter()
        .map(|i| (((i[0] as u32 + i[1] as u32 + i[2] as u32) / 3) as f32).round())
        .collect()
}

fn map_brightness_to_ascii(brightness_values: &[f32]) -> Vec<char> {
    brightness_values
        .iter()
        .map(|f| {
            let relative_brightness = 255.0 / f;
            let wanted_character =
                (ASCII_CHARACTERS.len() as f32 / relative_brightness).round() - 1.0;
            ASCII_CHARACTERS
                .chars()
                .nth(wanted_character as usize)
                .expect("index not found")
        })
        .collect()
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    if env_args.len() > 2 || env_args.len() < 2 {
        eprintln!("Invalid arguments\nUSAGE: cargo run <img_path>");
        return;
    }
    let mut img =
        image::open(env_args.get(1).expect("invalid index")).expect("could not load image");
    img = img.resize(
        img.width() / 2,
        img.height() / 2,
        image::imageops::FilterType::Gaussian,
    );

    println!("Image loaded");
    println!("{:?}", img.dimensions());

    let img_data = read_pixel_data(&img);
    let rgb_brightness_values = map_rgb_to_brightness(&img_data);
    let ascii_matrix = map_brightness_to_ascii(&rgb_brightness_values);
    for a in ascii_matrix.iter().enumerate() {
        if a.0 % img.width() as usize == 0 {
            println!();
        }
        print!("{}", a.1);
    }
}
