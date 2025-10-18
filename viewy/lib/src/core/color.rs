use hex::FromHex;

#[derive(Debug, Clone, Copy)]
pub struct RgbaColor(pub u8, pub u8, pub u8, pub u8);

impl RgbaColor {
    pub fn from_hex(hex: &str) -> Self {
        let color_string = hex.to_string().replace("#", "");
        let color_parsed = <[u8; 3]>::from_hex(color_string).expect("color format not valid");
        RgbaColor(color_parsed[0], color_parsed[1], color_parsed[2], 255)
    }

    pub fn from_alpha_hex(hex: &str) -> Self {
        let mut color_string = hex.to_string().replace("#", "");
        if color_string.len() == 6 {
            color_string = format!("{}ff", color_string);
        }
        let color_parsed = <[u8; 4]>::from_hex(color_string).expect("color format not valid");
        RgbaColor(
            color_parsed[0],
            color_parsed[1],
            color_parsed[2],
            color_parsed[3],
        )
    }

    pub fn to_string(&self) -> String {
        let color_array = [self.0, self.1, self.2];
        hex::encode(color_array)
    }
    pub fn to_css_value(&self) -> String {
        format!("#{}", self.to_string())
    }
}
