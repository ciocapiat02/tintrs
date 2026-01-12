use image;
pub struct Colorscheme {
    colors: Vec<image::Rgb<u8>>,
}

impl Colorscheme {
    pub fn new(colorscheme: Vec<image::Rgb<u8>>) -> Self {
        Colorscheme {
            colors: colorscheme.clone(),
        }
    }
    pub fn apply_to_mage(){}
    fn find_closest_color()-> u16 { return 1; }
    fn get_color_distance()-> u16 { return 1; }
    fn extract_palette_k_means(){}
}
