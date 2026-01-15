use image::{ImageBuffer};
use rand::Rng;
use cast::{u32};

pub struct Colorscheme {
    colors: Vec<image::Rgb<u8>>,
}

impl Colorscheme {
    pub fn from_colors(colorscheme: Vec<image::Rgb<u8>>) -> Self {
        Self {
            colors: colorscheme.clone(),
        }
    }

    pub fn from_image(image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>, k: usize, iterations:u32) -> Self {
        let new_colorscheme = Self::extract_palette_k_means(image, k, iterations);
        Self {
            colors: new_colorscheme,
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

    fn extract_palette_k_means(image: &ImageBuffer<image::Rgb<u8>, Vec<u8>>, k: usize, max_iterations: u32) -> Vec<image::Rgb<u8>>{
        let mut centroids: Vec<image::Rgb<u8>> = Vec::new();
        let mut rng = rand::rng();
        let pixels:Vec<&image::Rgb<u8>> = image.pixels().collect();
        for _ in 0..k {
            centroids.push(pixels[rng.random_range(0..pixels.len())].clone());
        }

        for _ in 0..max_iterations {
            let mut clusters: Vec<Vec<image::Rgb<u8>>> = vec![Vec::new(); k]; 
            for &pixel in pixels.clone() {
                let mut min_distance = u32::MAX;
                let mut min_index = 0;
                for (i, centroid) in centroids.iter().enumerate() {
                    let dist = Self::get_color_distance(centroid.clone(), pixel);
                    if dist < min_distance {
                        min_distance = dist;
                        min_index = i;
                    }
                }
                clusters[min_index].push(pixel);
            }

            let new_centroids: Vec<image::Rgb<u8>> = clusters.iter().enumerate().map(|(_i, cluster)| {
                if cluster.is_empty() {
                    return centroids[rng.random_range(0..centroids.len())];
                }

                let sum: [u32; 3] = cluster.iter().fold([0,0,0], |acc, pixel| {
                    [acc[0]+pixel[0] as u32, acc[1]+pixel[1] as u32, acc[2]+pixel[2] as u32]
                });

                let len:u32 = cluster.len() as u32;

                let new_centroid = image::Rgb::from([
                    (sum[0]/len) as u8,
                    (sum[1]/len) as u8,
                    (sum[2]/len) as u8,
                ]);
                return new_centroid;
            }).collect();

            if centroids == new_centroids {
                break;
            } else {
                centroids = new_centroids;
            }
        }
        
        return centroids;
    }

    pub fn get_colorscheme(&self) -> &Vec<image::Rgb<u8>> {
        return &self.colors;
    }

    pub fn gen_image_from_colorscheme(&self, block_size: u32) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let n = self.colors.len() as u32;
        let width = block_size * n;
        let height = block_size;
        
        let mut image = ImageBuffer::new(width, height);
        
        for (x, _y, pixel) in image.enumerate_pixels_mut() {
            let color_index = (x / block_size) as usize;
            *pixel = self.colors[color_index];
        }
        
        return image;
    }
}
