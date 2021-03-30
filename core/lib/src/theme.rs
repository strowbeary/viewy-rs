use indoc::formatdoc;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Colors {
    accent: Color,
    on_accent: Color,
    background: Color,
    on_background: Color,
    surface: Color,
    on_surface: Color,
    destructive: Color,
    on_destructive: Color
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Color {
    dark: String,
    light: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Theme {
   colors: Colors
}

impl Theme {
    pub fn to_scss(&self) -> String {
        let palette: String = formatdoc! {"
body.app-themes {{
    &--auto{{
        @media (prefers-color-scheme: light) {{
            @include palette(
                \"light\",
                {accent_light}, {on_accent_light},
                {background_light}, {on_background_light},
                {surface_light}, {on_surface_light},
                {destructive_light}, {on_destructive_light},
            );
        }}

        @media (prefers-color-scheme: dark) {{
            @include palette(
                \"dark\",
                 {accent_dark}, {on_accent_dark},
                {background_dark}, {on_background_dark},
                {surface_dark}, {on_surface_dark},
                {destructive_dark}, {on_destructive_dark},
            );
        }}
    }}

    &--light {{
        @include palette(
             \"light\",
            {accent_light}, {on_accent_light},
            {background_light}, {on_background_light},
            {surface_light}, {on_surface_light},
            {destructive_light}, {on_destructive_light},
        );
    }}

    &--dark {{
        @include palette(
            \"dark\",
            {accent_dark}, {on_accent_dark},
            {background_dark}, {on_background_dark},
            {surface_dark}, {on_surface_dark},
            {destructive_dark}, {on_destructive_dark},
        );
    }}
}}
        ",
                accent_light = self.colors.accent.light,
                on_accent_light = self.colors.on_accent.light,

                background_light = self.colors.background.light,
                on_background_light = self.colors.on_background.light,

                surface_light = self.colors.surface.light,
                on_surface_light = self.colors.on_surface.light,

                destructive_light = self.colors.destructive.light,
                on_destructive_light = self.colors.on_destructive.light,

                accent_dark = self.colors.accent.dark,
                on_accent_dark = self.colors.on_accent.dark,

                background_dark = self.colors.background.dark,
                on_background_dark = self.colors.on_background.dark,

                surface_dark = self.colors.surface.dark,
                on_surface_dark = self.colors.on_surface.dark,

                destructive_dark = self.colors.destructive.dark,
                on_destructive_dark = self.colors.on_destructive.dark,
        };
        palette
    }
}