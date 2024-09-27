use figment::providers::{Env, Format, Toml};
use figment::value::{Dict, Map};
use figment::{Error, Figment, Metadata, Profile, Provider};
use hex::FromHex;
use palette::Srgba;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, Copy, Debug)]
pub struct HexColor(pub [u8; 4]);

impl From<&str> for HexColor {
    fn from(value: &str) -> Self {
        Self(
            <[u8; 4]>::from_hex(format!("{:f<8}", value.trim().replace("#", "")))
                .unwrap_or([0, 0, 0, 0]),
        )
    }
}

impl From<String> for HexColor {
    fn from(value: String) -> Self {
        Self(
            <[u8; 4]>::from_hex(format!("{:f<8}", value.trim().replace("#", "")))
                .unwrap_or([0, 0, 0, 0]),
        )
    }
}

impl From<Srgba<u8>> for HexColor {
    fn from(value: Srgba<u8>) -> Self {
        Self([value.red, value.green, value.blue, value.alpha])
    }
}

impl ToString for HexColor {
    fn to_string(&self) -> String {
        format!("#{}", hex::encode(self.0))
    }
}

impl Into<Srgba<u8>> for HexColor {
    fn into(self) -> Srgba<u8> {
        Srgba::<u8>::from(self.0)
    }
}
impl Into<Srgba<f64>> for HexColor {
    fn into(self) -> Srgba<f64> {
        Srgba::<f64>::from([self.0[0] as f64, self.0[1] as f64, self.0[2] as f64, self.0[3] as f64, ])
    }
}

impl Into<Srgba<f32>> for HexColor {
    fn into(self) -> Srgba<f32> {
        Srgba::<f32>::from([self.0[0] as f32, self.0[1] as f32, self.0[2] as f32, self.0[3] as f32, ])
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<HexColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        let color_string = String::deserialize(deserializer)?;

        Ok(HexColor::from(color_string.as_str()))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub name: String,
    pub favicons: Vec<Favicon>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Favicon {
    pub rel: String,
    pub href: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Colors {
    pub accent: ThemedColor,
    pub background: ThemedColor,
    pub surface: ThemedColor,
    pub destructive: ThemedColor,
    pub success: ThemedColor,
    pub warning: ThemedColor,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThemedColor {
    pub dark: HexColor,
    pub light: HexColor,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shapes {
    #[serde(rename = "border-radius")]
    pub border_radius: i32,
    #[serde(rename = "spacing-factor")]
    pub spacing_factor: i32,
}

/// App and theme configuration
///
/// # Viewy.toml breakdown
/// The `Viewy.toml` configuration file provides a comprehensive way to customize and configure various aspects of the Viewy UI toolkit for your application. Here's a breakdown of the provided file:
/// ```toml
/// [app]
/// name = "Viewy showcase"
/// favicons = [
///     { rel = "shortcut icon", href="/assets/favicon.svg"}
/// ]
///
/// [colors]
/// accent = { light = "#0052cc", dark = "#3385ff" }
///
/// background = { light = "#ffffff", dark = "#121212" }
///
/// surface = { light = "#efefef", dark = "#181818" }
///
/// destructive = { light = "#C70039 ", dark = "#ff0048" }
///
/// success = { light = "#3DA144 ", dark = "#3DA144" }
///
/// [shapes]
/// border-radius = 8
/// spacing-factor = 4
/// ```
///
/// #### [app]
///
/// This section contains general application settings.
///
/// - `name`: The name of the application, in this case, "Viewy showcase".
///
/// - `favicons`: A list of favicons for the web application. These are typically displayed in the browser tab next to the title. The file here provides a path to an SVG file to be used as a favicon.
///
/// #### [colors]
///
/// Here, you can define the color schemes for various UI components. The properties are broken down into light and dark themes. Each color definition has two parts:
///
/// - The main color (e.g., `accent`, `background`, `surface`, `destructive`, `success`).
///
/// #### [shapes]
///
/// This section defines the geometric properties of components:
///
/// - `border-radius`: Determines the curvature of edges for components like buttons, cards, etc. A value of 8 implies a subtle curve.
///
/// - `spacing-factor`: Defines the factor by which spaces between UI components are multiplied. A factor of 4 can be used to derive consistent spacings like 4, 8, 12, 16, etc.
///
/// ---
///
/// Using the `Viewy.toml` file, developers can ensure that their applications maintain a consistent look and feel across different pages and components. It acts as a central place to modify and adapt the visual language of the application according to branding or aesthetic requirements.
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub app: AppSettings,
    pub colors: Colors,
    pub shapes: Shapes,
}

impl Config {
    pub fn load() -> Self {
        let figment = Figment::from({
            Figment::new()
                .merge(Config::default())
                .merge(Toml::file("Viewy.toml"))
                .merge(Toml::file("viewy.toml"))
        })
        .merge(Env::prefixed("VIEWY_").split("_"));

        let config = figment
            .extract::<Config>()
            .map_err(|err| {
                println!("Viewy config error {:?}", err);
                err
            })
            .unwrap();
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app: AppSettings {
                name: "My Viewy App".to_string(),
                favicons: vec![],
            },
            colors: Colors {
                accent: ThemedColor {
                    dark: "#3385ff".into(),
                    light: "#0052cc".into(),
                },

                background: ThemedColor {
                    dark: "#121212".into(),
                    light: "#ffffff".into(),
                },

                surface: ThemedColor {
                    dark: "#181818".into(),
                    light: "#efefef".into(),
                },

                destructive: ThemedColor {
                    dark: "#ff0048".into(),
                    light: "#C70039".into(),
                },

                success: ThemedColor {
                    dark: "#3DA144".into(),
                    light: "#3DA144".into(),
                },
                warning: ThemedColor {
                    dark: "#FFB073".into(),
                    light: "#DF7B5E".into(),
                },
            },
            shapes: Shapes {
                border_radius: 8,
                spacing_factor: 4,
            },
        }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Viewy Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        // Optionally, a profile that's selected by default.
        None
    }
}
