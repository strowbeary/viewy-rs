use grass::OutputStyle;
use palette::Srgba;
use palette::color_difference::Wcag21RelativeContrast;
use strum::IntoEnumIterator;

pub use colors::Color;

use crate::core::config::HexColor;
use crate::widgets::get_all_stylesheet;

mod colors;
mod fonts;
mod sizing;

#[derive(Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Auto => "auto",
        }
    }
}

fn negative_contrast(color: HexColor) -> HexColor {
    let color: Srgba<u8> = color.into();
    let real_color = Srgba::<f64>::from_format(color);

    if real_color.relative_luminance().luma > 0.5 {
        HexColor([0, 0, 0, 255])
    } else {
        HexColor([255, 255, 255, 255])
    }
}

fn generate_color_palette(theme_variant: Theme) -> String {
    let palette = Color::iter()
        .map(|color| {
            format!(
                "{}: light-dark({}, {});",
                color.as_str(),
                color.get_hex_color(&Theme::Light).to_string(),
                color.get_hex_color(&Theme::Dark).to_string()
            )
        })
        .collect::<Vec<String>>()
        .join("");

    let color_scheme_param = match theme_variant {
        Theme::Dark => "color-scheme: dark;",
        Theme::Light => "color-scheme: light;",
        Theme::Auto => "color-scheme: light dark;",
    };
    format!(
        r#"
    :root {{
     {color_scheme_param}
     {palette}
    }}
    "#
    )
}

pub fn get_stylesheet() -> String {
    let palette_style = generate_color_palette(Theme::Auto);
    let widget_style = get_all_stylesheet().join("");

    grass::from_string(
        format!(
            r#"


        {palette_style}


        {widget_style}
        "#
        ),
        &grass::Options::default().style(OutputStyle::Compressed),
    )
        .expect("Can't compile SCSS")
}
