use hex::FromHex;
use uuid::Uuid;

use crate::components::{Alignment, Appendable, HStack, Text, TextStyle, View};
use crate::node::Node;
use crate::{DefaultModifiers, Renderable};

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let color_string = hex.to_string().replace("#", "");
        let color_parsed = <[u8; 3]>::from_hex(color_string).expect("color format not valid");
        Color(color_parsed[0], color_parsed[1], color_parsed[2], 255)
    }

    pub fn from_alpha_hex(hex: &str) -> Self {
        let mut color_string = hex.to_string().replace("#", "");
        if color_string.len() == 6 {
            color_string = format!("{}ff", color_string);
        }
        let color_parsed = <[u8; 4]>::from_hex(color_string).expect("color format not valid");
        Color(
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

#[derive(Debug, Clone)]
pub enum ColorPickerStyle {
    Palette(Vec<Color>),
    Free(Option<Color>),
}

impl ColorPickerStyle {
    pub fn get_style_name(&self) -> String {
        match self {
            ColorPickerStyle::Palette(_) => "palette",
            ColorPickerStyle::Free(_) => "free",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ColorPicker {
    node: Node,
    style: ColorPickerStyle,
    label: Option<String>,
    name: String,
}

impl ColorPicker {
    pub fn new(name: &str, style: ColorPickerStyle) -> Self {
        ColorPicker {
            node: Default::default(),
            style,
            label: None,
            name: name.to_string(),
        }
    }
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }
}

impl std::ops::Deref for ColorPicker {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for ColorPicker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for ColorPicker {}

impl Renderable for ColorPicker {
    fn render(mut self) -> Node {
        let base_class = format!("color-picker--{}", self.style.get_style_name());
        self.add_class("color-picker").add_class(&base_class);
        if let Some(label) = &self.label {
            self.node
                .children
                .push({ Text::new(label, TextStyle::Label) }.render());
        }
        let picker_id = Uuid::new_v4().to_string();
        match &self.style {
            ColorPickerStyle::Palette(color_palette) => self.node.children.push({
                let mut option_list = HStack::new(Alignment::Center);
                option_list.add_class(&format!("{}--option-list", &base_class));
                for color in color_palette {
                    let radio_id = format!("color-picker-{}-{}", picker_id, color.to_string());
                    option_list.append_child({
                        let mut input = View::new();
                        input
                            .tag("input")
                            .set_attr("type", "radio")
                            .set_attr("name", self.name.as_str())
                            .set_attr("value", &format!("#{}", color.to_string()))
                            .set_attr("id", radio_id.as_str())
                            .add_class(&format!("{}--option-list--radio", &base_class));
                        input
                    });
                    option_list.append_child({
                        let mut label = View::new();
                        label
                            .tag("label")
                            .set_attr("for", radio_id.as_str())
                            .color(&color.to_css_value())
                            .add_class(&format!("{}--option-list--swatch", &base_class));
                        label
                    });
                }
                option_list.render()
            }),
            ColorPickerStyle::Free(color) => {
                let color_hex_code = format!(
                    "#{}",
                    color.unwrap_or(Color::from_hex("#ffffff")).to_string()
                );
                self.node.children.push({
                    let mut option_list = View::new();
                    option_list
                        .tag("input")
                        .set_attr("name", self.name.as_str())
                        .set_attr("value", { &color_hex_code })
                        .set_attr("type", "color")
                        .add_class(&format!("{}--input", &base_class));

                    option_list.render()
                })
            }
        }
        self.node
    }
}
