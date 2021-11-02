use std::path::Path;
use std::fs::File;
use std::{env, fs};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Colors {
    pub accent: Color,
    #[serde(rename = "on-accent")]
    pub on_accent: Color,
    pub background: Color,
    #[serde(rename = "on-background")]
    pub on_background: Color,
    pub surface: Color,
    #[serde(rename = "on-surface")]
    pub on_surface: Color,
    pub destructive: Color,
    #[serde(rename = "on-destructive")]
    pub on_destructive: Color,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    pub dark: String,
    pub light: String,
}

#[derive(Deserialize, Serialize)]
pub struct Shapes {
    #[serde(rename = "border-radius")]
    pub border_radius: i32,
    #[serde(rename = "spacing-factor")]
    pub spacing_factor: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Theme {
    pub colors: Colors,
    pub shapes: Shapes,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            colors: Colors {
                accent: Color {
                    dark: "#1d5dea".to_string(),
                    light: "#1d5dea".to_string(),
                },
                on_accent: Color {
                    dark: "#ffffff".to_string(),
                    light: "#ffffff".to_string()
                },
                background: Color {
                    dark: "#121212".to_string(),
                    light: "#ffffff".to_string()
                },
                on_background: Color {
                    dark: "#ffffff".to_string(),
                    light: "#000000".to_string()
                },
                surface: Color {
                    dark: "#181818".to_string(),
                    light: "#efefef".to_string()
                },
                on_surface: Color {
                    dark: "#ffffff".to_string(),
                    light: "#000000".to_string()
                },
                destructive: Color {
                    dark: "#f14a61".to_string(),
                    light: "#f14a61".to_string()
                },
                on_destructive: Color {
                    dark: "#ffffff".to_string(),
                    light: "#ffffff".to_string()
                },
            },
            shapes: Shapes {
                border_radius: 8,
                spacing_factor: 4
            }
        }
    }
}

pub struct ThemeLoader;

impl ThemeLoader {
    pub fn load_from_config_folder() -> Theme {
        let theme_path = Path::new("./.viewy/theme.toml");
        let mut theme = if theme_path.exists() {
            fs::read_to_string(theme_path)
                .map(|theme_config| -> Theme {
                    toml::from_str(&theme_config).expect("Can't parse theme config file")
                })
                .expect("Can't open config file")
        } else {
            Theme::default()
        };

        theme.colors.accent.light = env::var("VIEWY_COLOR_ACCENT_LIGHT")
            .unwrap_or(theme.colors.accent.light);
        theme.colors.accent.dark = env::var("VIEWY_COLOR_ACCENT_DARK")
            .unwrap_or(theme.colors.accent.dark);

        theme
    }
}