use crate::components::Color;
use palette::*;
use palette::luma::Luma;

const PHI: f32 = 1.61803398 * 1.3;

/// Function to convert pixel values into rem (factor of browser's default font size)
pub fn sp(value: i32) -> String {
    let real_value = value as f32;
    format!("{}rem", real_value / 16.0)
}

/// Helper to get harmonious spacing and sizing values.
/// It's based on `scale ^ PHI` calculation
pub fn scale(scale: i32) -> i32 {
    let real_scale = scale as f32;
    real_scale.powf(PHI).ceil() as i32
}

/// Helper to get most visible text color based on a background color
/// Return black if luminance is less than 50% and white otherwise
pub fn contrast_color(color_hex: &String) -> String {
    let color = Color::from_hex(&color_hex);
    let lcolor: SrgbLuma  =  Srgb::new(
        color.0 as f32 / 255.0,
        color.1 as f32 / 255.0,
        color.2 as f32 / 255.0
    ).into_color();
    if lcolor.luma > 0.5 {
        "black".to_string()
    } else {
        "white".to_string()
    }
}