use grass::OutputStyle;
use grass_compiler::Result as SassResult;
use grass_compiler::sass_ast::ArgumentResult;
use grass_compiler::sass_value::{SassNumber, Unit, Value};
use grass_compiler::{Builtin, Visitor};
use palette::Srgba;
use palette::color_difference::Wcag21RelativeContrast;
use std::sync::OnceLock;
use strum::IntoEnumIterator;

pub use colors::Color;

use crate::CONFIG;
use crate::core::config::HexColor;
use crate::widgets::get_all_stylesheet;

mod colors;
mod fonts;
mod sizing;

static COMPILED_STYLESHEET: OnceLock<String> = OnceLock::new();

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

pub fn grass_scale(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;
    let scale_arg = args
        .get(0, "scale")
        .expect("$scale argument must be defined")
        .node;
    if let Value::Dimension(num) = scale_arg {
        let real_scale = num.num.0;
        let real_spacing_factor = CONFIG.shapes.spacing_factor as f64;
        let rem_val = real_scale.powf(real_spacing_factor.sqrt()).ceil() / 16.0;
        Ok(Value::Dimension(SassNumber {
            num: rem_val.into(),
            unit: Unit::Rem,
            as_slash: None,
        }))
    } else {
        panic!("$scale argument must be a dimension")
    }
}

pub fn grass_sp(mut args: ArgumentResult, visitor: &mut Visitor) -> SassResult<Value> {
    args.max_args(1)?;
    let scale_arg = args
        .get(0, "sp")
        .expect("$sp argument must be defined")
        .node;
    if let Value::Dimension(num) = scale_arg {
        let real_value = num.num.0;
        let rem_val = real_value / 16.0;
        Ok(Value::Dimension(SassNumber {
            num: rem_val.into(),
            unit: Unit::Rem,
            as_slash: None,
        }))
    } else {
        panic!("$sp argument must be a dimension")
    }
}

fn get_stylesheet_cached() -> &'static str {
    COMPILED_STYLESHEET
        .get_or_init(|| {
            let palette_style = generate_color_palette(Theme::Auto);
            let widget_style = get_all_stylesheet().join("");
            let options = grass::Options::default()
                .style(OutputStyle::Compressed)
                .add_custom_fn("sp", Builtin::new(grass_sp))
                .add_custom_fn("scale", Builtin::new(grass_scale));
            grass::from_string(
                format!(
                    r#"


        {palette_style}


        {widget_style}
        "#
                ),
                &options,
            )
            .expect("Can't compile SCSS")
        })
        .as_str()
}

pub fn get_stylesheet() -> String {
    get_stylesheet_cached().to_owned()
}
