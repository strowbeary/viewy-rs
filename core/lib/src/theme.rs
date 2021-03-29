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

struct Color {
    dark: String,
    light: String
}

pub struct Theme {
   colors: Colors
}
