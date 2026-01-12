use image::{ImageBuffer};
use cast::{u32};

pub struct Colorscheme {
    colors: Vec<image::Rgb<u8>>,
}

impl Colorscheme {
    pub fn new(colorscheme: Vec<image::Rgb<u8>>) -> Self {
        Self {
            colors: colorscheme.clone(),
        }
    }
    pub fn apply_to_image(&self, image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> ImageBuffer<image::Rgb<u8>, Vec<u8>>{
        let mut new_image = image.clone();

        for (_x, _y, pixel) in new_image.enumerate_pixels_mut() {
            let index = Self::find_closest_color(&self, *pixel);
            *pixel = self.colors[index];
        }

        return new_image;
    }
    
    fn find_closest_color(&self, input_color: image::Rgb<u8>)-> usize { 
        let mut min_distance = 255*3;
        let mut min_index = 0;
        for color_index in 0..self.colors.len() {
            let distance = Self::get_color_distance(self.colors[color_index], input_color);
            if distance < min_distance {
                min_distance = distance;
                min_index = color_index; 
            }
        }
        return min_index;
    }

    fn get_color_distance(color1: image::Rgb<u8>, color2: image::Rgb<u8>)-> u32 { 
        u32((color1[0] as i16 - color2[0] as i16).abs()+
        (color1[1] as i16 - color2[1] as i16).abs()+
        (color1[2] as i16 - color2[2] as i16).abs()).unwrap()
    }
    fn extract_palette_k_means(){}
}
