const PHI: f32 = 1.61803398875;

pub fn sp(value: i32) -> String {
    let real_value = value as f32;
    format!("{}rem", real_value / 16.0)
}

pub fn scale(scale: i32) -> i32 {
    let real_scale = scale as f32;
    real_scale.powf(PHI).ceil() as i32
}