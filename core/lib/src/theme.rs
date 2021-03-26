struct Palette {
    primary: Color,
    on_primary: Color,
    background: Color,
    on_background: Color,
    surface: Color,
    destructive: Color
}

struct ColorTheme {
    dark: Palette,
    light: Palette
}

pub struct Theme {
   colors: ColorTheme
}