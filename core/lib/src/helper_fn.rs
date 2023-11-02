use palette::*;
use crate::core::color::RgbaColor;

const PHI: f64 = 1.61803398875;

/// Converts a pixel value to its relative `rem` unit representation in CSS.
///
/// This function takes an integer value representing pixels, and then converts it into a floating point
/// representation of its `rem` equivalent by dividing by 16. This is based on the common practice where
/// 16 pixels is considered equivalent to 1 `rem` (assuming the root font size is set to 16px).
///
/// # Arguments
///
/// * `value`: An `i32` representing the pixel value to be converted.
///
/// # Returns
///
/// Returns a `String` which represents the `rem` equivalent of the provided pixel value.
///
/// # Examples
///
/// ```
/// let pixel_value = 32;
/// let rem_representation = sp(pixel_value);
/// println!("{}", rem_representation); // Outputs "2rem"
/// ```
pub fn sp(value: i32) -> String {
    let real_value = value as f32;
    format!("{}rem", real_value / 16.0)
}

/// Scales a given value using the power of the golden ratio (PHI).
///
/// This function raises the golden ratio (PHI) to the power of the provided scale (converted to a float)
/// and then takes the ceiling of the result. The resulting value is then converted back to an `i32`.
///
/// # Arguments
///
/// * `scale`: An `i32` representing the value to be scaled using the golden ratio.
///
/// # Returns
///
/// Returns an `i32` which is the ceiling value of the golden ratio raised to the power of the provided scale.
///
/// # Examples
///
/// ```
/// let value = 2;
/// let scaled_value = scale(value);
/// println!("{}", scaled_value); // Outputs the scaled value using the golden ratio
/// ```
pub fn scale(scale: i32) -> i32 {
    let real_scale = scale as f64;
    real_scale.powf(PHI).ceil() as i32
}

/// Determines a contrasting text color (either black or white) based on the luminance of the input color.
///
/// The function calculates the luminance of the provided color and returns "#000000" (black) if the luminance is high,
/// or "#ffffff" (white) if the luminance is low. This is intended to provide a suitable text color that contrasts well
/// with the background color to ensure readability.
///
/// # Arguments
///
/// * `color_hex`: A `&String` reference to the hexadecimal representation of the color.
///
/// # Returns
///
/// Returns a `String` containing the hexadecimal representation of the contrasting text color,
/// which is either "#000000" (black) or "#ffffff" (white).
///
/// # Examples
///
/// ```
/// let background_color = String::from("#F0F0F0"); // Light gray background
/// let text_color = contrast_color(&background_color);
/// println!("{}", text_color); // Outputs either "#000000" (black) or "#ffffff" (white) based on background color luminance
/// ```
pub fn contrast_color(color_hex: &String) -> String {
    let color = RgbaColor::from_alpha_hex(&color_hex);
    let lcolor: SrgbLuma  =  Srgba::new(
        color.0 as f32 / 255.0,
        color.1 as f32 / 255.0,
        color.2 as f32 / 255.0,
        color.3 as f32 / 255.0
    ).into_color();
    if lcolor.luma > 0.5 {
        "#000000".to_string()
    } else {
        "#ffffff".to_string()
    }
}

// Modifies a color based on its alpha (transparency) value, making it closer to white the more transparent it is.
///
/// The function expects a hexadecimal representation of a color in the format "#RRGGBBAA" or "#RRGGBB".
/// If the alpha channel is absent or at full opacity, the original color will be returned.
/// For partially transparent colors, the resulting color will be a mix between the original color and white,
/// where the degree of mixing is determined by the transparency level.
///
/// # Arguments
///
/// * `color_hex`: A `&String` reference to the hexadecimal representation of the color.
///
/// # Returns
///
/// Returns a `String` containing the hexadecimal representation of the modified color, in the format "#RRGGBB".
///
/// # Examples
///
/// ```
/// let color = String::from("#FF000080"); // Half-transparent red
/// let new_color = harden_color(&color);
/// println!("{}", new_color); // Outputs a color that is closer to white than the original red
/// ```
pub fn harden_color(color_hex: &String) -> String {
    let color = RgbaColor::from_alpha_hex(&color_hex);
    let alpha = color.3 as f32 / 255.0;
    let new_color = LinSrgb::new(
        color.0 as f32 / 255.0,
        color.1 as f32 / 255.0,
        color.2 as f32 / 255.0,
    )
        .mix(LinSrgb::new(1.0, 1.0, 1.0), 1.0 - alpha);
    format!("#{:02x?}{:02x?}{:02x?}", (new_color.red * 255.0) as u8, (new_color.green * 255.0) as u8, (new_color.blue * 255.0) as u8)
}