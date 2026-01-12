use image;
pub fn rgb_to_hex(rgb_value: image::Rgb<u8>) -> String {
    let [r, g, b] = rgb_value.0;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn hex_to_rgb(hex_value: &str) -> image::Rgb<u8> {
    let hex = hex_value.trim_start_matches('#');
    
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    
    image::Rgb([r, g, b])
}
