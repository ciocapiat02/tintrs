mod args;
mod colorscheme;
mod effect;
mod format_converter;

use args::Args;
use format_converter::{hex_to_rgb};
use colorscheme::Colorscheme;
use image::{ImageBuffer, ImageReader};
use yaml_rust2::{YamlLoader};
use clap::Parser;

fn load_colorscheme(path: &String) -> Vec<image::Rgb<u8>>{
    let mut output = Vec::new();

    let colorscheme_file_content = std::fs::read_to_string(&path).expect(&format!("error opening file {}", path));
    let colorscheme_yaml = &YamlLoader::load_from_str(colorscheme_file_content.as_str())
        .expect(&format!("file {} is not a yaml file", &path));
    let colorscheme_vec = match colorscheme_yaml[0]["colorscheme"].as_vec() {
        Some(colorscheme) => colorscheme,
        None => panic!("colorscheme does not contain colors"),
    };
    for color in colorscheme_vec {
        let color_str = match color.as_str() {
            Some(color) => color,
            None => panic!("could not read color"),
        };
        output.push(hex_to_rgb(color_str));
    }

    return output;
}

fn save_image(image: ImageBuffer<image::Rgb<u8>, Vec<u8>>, path: String) {
    match image.save(path){
        Err(path) => panic!("could not save image to: {}", path),
        _ => ()
    }
}

fn main() {
    let args = Args::parse();
    let input_path = args.input.clone(); 
    let input_image = ImageReader::open(input_path)
        .expect(&format!("Could not open file: {}", args.input))
        .decode()
        .expect(&format!("Could not decode image: {}", args.input))
        .to_rgb8();

    let mut output_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(input_image.dimensions().0, input_image.dimensions().1);
    if args.action == "extract" {
        println!("Extracting {} colors from {} and saving to {}", args.length, args.input, args.output);
        // Call extract function here
    }

    else if args.action == "apply" {
        let colorscheme_vec: Vec<image::Rgb<u8>> = match args.colorscheme {
            Some(colorscheme_path) => load_colorscheme(&colorscheme_path),
            None => panic!("You need to specify a colorscheme for it to be applyed"),
        };
        let colorscheme = Colorscheme::new(colorscheme_vec); 
        output_image = colorscheme.apply_to_image(&input_image);
    } 

    else if args.action == "blur" {
        println!("Applying blur of amount {} to {} and saving to {}", args.blur_amount, args.input, args.output);
    }

    else {
        eprintln!("Unknown action: {}", args.action);
    } 
    save_image(output_image, args.output);
}
