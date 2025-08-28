use std::{env, error::Error};
fn get_std_in() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Invalid usage: ./ascii <image_path>".to_string().into());
    }
    Ok(args[1].clone())

}
fn get_file(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let image = image::open(path).expect("Error while opening the file");
    let rgb_image = image.to_rgb8();
    let pixels: Vec<u8> = rgb_image.as_raw().to_vec();
    Ok(pixels)
}
fn main() -> Result<(), Box<dyn Error>>{
    let path = get_std_in()?;
    let file = get_file(&path)?;
    println!("file: \n{:?}", file);
    Ok(())
}
