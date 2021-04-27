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