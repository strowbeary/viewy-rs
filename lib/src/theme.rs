use std::{env, fs};
use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AppSettings {
    pub name: String,
    pub favicons: Vec<Favicon>,
}

#[derive(Deserialize, Serialize)]
pub struct Favicon {
    pub rel: String,
    pub href: String,
}

#[derive(Deserialize, Serialize)]
pub struct Features {
    #[serde(rename = "rich-text-editor")]
    pub rich_text_editor: bool,
    #[serde(rename = "sortable-stack")]
    pub sortable_stack: bool,
}

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
    pub success: Color,
    #[serde(rename = "on-success")]
    pub on_success: Color,
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
pub struct Config {
    pub app: AppSettings,
    pub features: Features,
    pub colors: Colors,
    pub shapes: Shapes,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app: AppSettings {
                name: "".to_string(),
                favicons: vec![],
            },
            features: Features {
                rich_text_editor: false,
                sortable_stack: false,
            },
            colors: Colors {
                accent: Color {
                    dark: "#1d5dea".to_string(),
                    light: "#1d5dea".to_string(),
                },
                on_accent: Color {
                    dark: "#ffffff".to_string(),
                    light: "#ffffff".to_string(),
                },
                background: Color {
                    dark: "#121212".to_string(),
                    light: "#ffffff".to_string(),
                },
                on_background: Color {
                    dark: "#ffffff".to_string(),
                    light: "#000000".to_string(),
                },
                surface: Color {
                    dark: "#181818".to_string(),
                    light: "#efefef".to_string(),
                },
                on_surface: Color {
                    dark: "#ffffff".to_string(),
                    light: "#000000".to_string(),
                },
                destructive: Color {
                    dark: "#f14a61".to_string(),
                    light: "#f14a61".to_string(),
                },
                on_destructive: Color {
                    dark: "#ffffff".to_string(),
                    light: "#ffffff".to_string(),
                },
                success: Color { dark: "#3DA144".to_string(), light: "#3DA144".to_string() },
                on_success: Color { dark: "#ffffff".to_string(), light: "#ffffff".to_string() },
            },
            shapes: Shapes {
                border_radius: 8,
                spacing_factor: 4,
            },
        }
    }
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_config_folder() -> Config {
        let theme_path = Path::new("./viewy.toml");
        let mut theme = if theme_path.exists() {
            fs::read_to_string(theme_path)
                .map(|theme_config| -> Config {
                    toml::from_str(&theme_config).expect("Can't parse theme config file")
                })
                .expect("Can't open config file")
        } else {
            Config::default()
        };

        theme.colors.accent.light = env::var("VIEWY_COLOR_ACCENT_LIGHT")
            .unwrap_or(theme.colors.accent.light);
        theme.colors.accent.dark = env::var("VIEWY_COLOR_ACCENT_DARK")
            .unwrap_or(theme.colors.accent.dark);

        theme
    }
}