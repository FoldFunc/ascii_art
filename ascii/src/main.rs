use std::env;
use std::error::Error;

/// Get the image path from command-line arguments
fn get_std_in() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Invalid usage: ./ascii <image_path>".to_string().into());
    }
    Ok(args[1].clone())
}

/// Load the image and return a flat RGB buffer along with width and height
fn get_file(path: &str) -> Result<(Vec<u8>, u32, u32), Box<dyn Error>> {
    let image = image::open(path)?;
    let width = image.width();
    let height = image.height();
    let rgb_image = image.to_rgb8();
    let pixels: Vec<u8> = rgb_image.into_raw(); // flat RGB buffer
    Ok((pixels, width, height))
}

/// Resize the image to target width/height using nearest neighbor
fn scale(
    file: &Vec<u8>,
    target_width: u32,
    target_height: u32,
    original_w: u32,
    original_h: u32,
) -> Vec<u8> {
    let mut target_file: Vec<u8> = Vec::with_capacity((target_width * target_height * 3) as usize);

    for j in 0..target_height {
        for i in 0..target_width {
            let sx = (i * original_w / target_width) as usize;
            let sy = (j * original_h / target_height) as usize;
            let idx = (sy * original_w as usize + sx) * 3;
            target_file.push(file[idx]);     // R
            target_file.push(file[idx + 1]); // G
            target_file.push(file[idx + 2]); // B
        }
    }

    target_file
}

/// Convert RGB buffer to luminance (0..255)
fn luminance(file: &Vec<u8>) -> Vec<u8> {
    let mut lum: Vec<u8> = Vec::with_capacity(file.len() / 3);
    for chunk in file.chunks(3) {
        if chunk.len() < 3 {
            continue;
        }
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let l = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32).round();
        lum.push(l.clamp(0.0, 255.0) as u8);
    }
    lum
}

/// Map 0..255 luminance to ASCII characters
fn map_ascii(luminances: &Vec<u8>) -> Vec<char> {
    let charset = [' ', '`', '.', ',', '-', '+', '*', '!', '$', '#'];
    let mut result_ascii: Vec<char> = Vec::with_capacity(luminances.len());

    for &l in luminances {
        let idx = ((l as f32 / 255.0) * (charset.len() as f32 - 1.0)).round() as usize;
        result_ascii.push(charset[idx]);
    }

    result_ascii
}

/// Convert a 1D ASCII vector to a multi-line string
fn vec_to_ascii_image(output: &Vec<char>, width: u32, height: u32) -> String {
    let mut s = String::with_capacity((width * height + height) as usize);
    for j in 0..height {
        for i in 0..width {
            let idx = (j * width + i) as usize;
            s.push(output[idx]);
        }
        s.push('\n');
    }
    s
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_std_in()?;
    let (file, original_w, original_h) = get_file(&path)?;

    // Target width in characters
    let target_width = 118;
    let target_height_f32 = original_h as f32 * (target_width as f32 / original_w as f32) * 0.55;
    let target_height = target_height_f32.round() as u32;

    // Resize the image
    let target_file = scale(&file, target_width, target_height, original_w, original_h);

    // Convert to luminance
    let luminances = luminance(&target_file);

    // Map to ASCII
    let output_vec = map_ascii(&luminances);

    // Build final ASCII string
    let ascii_image = vec_to_ascii_image(&output_vec, target_width, target_height);

    // Print to screen
    println!("{}", ascii_image);

    Ok(())
}
