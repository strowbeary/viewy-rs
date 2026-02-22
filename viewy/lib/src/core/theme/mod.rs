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

fn parse_usize_after(haystack: &str, marker: &str) -> Option<usize> {
    let start = haystack.find(marker)? + marker.len();
    let digits = haystack[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<usize>().ok()
    }
}

fn parse_grass_error_location(error_debug: &str) -> Option<(usize, usize)> {
    let line = parse_usize_after(error_debug, "begin: LineCol { line: ")?;
    let column = parse_usize_after(error_debug, "column: ")?;
    Some((line, column))
}

fn scss_excerpt(source: &str, line: usize, column: usize, context_lines: usize) -> String {
    let lines = source.lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        return String::from("<empty stylesheet>");
    }

    let target_line = line.clamp(1, lines.len());
    let from = target_line.saturating_sub(context_lines).max(1);
    let to = (target_line + context_lines).min(lines.len());

    let mut out = String::new();
    for line_number in from..=to {
        let prefix = if line_number == target_line {
            ">>"
        } else {
            "  "
        };
        let content = lines[line_number - 1];
        out.push_str(&format!("{prefix} {line_number:>5} | {content}\n"));
        if line_number == target_line {
            let caret_padding = " ".repeat(column.saturating_sub(1));
            out.push_str(&format!("   {:>5} | {caret_padding}^\n", ""));
        }
    }
    out
}

fn format_grass_compile_error(error: &grass::Error, source: &str) -> String {
    let error_debug = format!("{error:?}");
    if let Some((line, column)) = parse_grass_error_location(&error_debug) {
        let excerpt = scss_excerpt(source, line, column, 3);
        format!(
            "Can't compile SCSS.\n{error}\nLocation: line {line}, column {column}\n\nSCSS excerpt:\n{excerpt}\nDebug:\n{error_debug}"
        )
    } else {
        format!("Can't compile SCSS.\n{error}\nDebug:\n{error_debug}")
    }
}

fn get_stylesheet_cached() -> &'static str {
    COMPILED_STYLESHEET
        .get_or_init(|| {
            let palette_style = generate_color_palette(Theme::Auto);
            let widget_style = get_all_stylesheet().join("");
            let full_style = format!(
                r#"
        {palette_style}

        {widget_style}
        "#
            );

            let options = grass::Options::default()
                .style(OutputStyle::Compressed)
                .add_custom_fn("sp", Builtin::new(grass_sp))
                .add_custom_fn("scale", Builtin::new(grass_scale));
            grass::from_string(full_style.clone(), &options).unwrap_or_else(|error| {
                panic!("{}", format_grass_compile_error(&error, &full_style))
            })
        })
        .as_str()
}

pub fn get_stylesheet() -> String {
    get_stylesheet_cached().to_owned()
}
