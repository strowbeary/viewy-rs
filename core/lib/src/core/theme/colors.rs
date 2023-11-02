use std::slice::Iter;

#[derive(Copy, Clone)]
pub enum Color {
    Accent,
    OnAccent,
    Background,
    OnBackground,
    SurfaceDim,
    Surface,
    OnSurface,
    SurfaceBright,
    AccentuatedSurfaceDim,
    AccentuatedSurface,
    AccentuatedSurfaceBright,
    Border,
    Success,
    SuccessSurface,
    Destructive,
    DestructiveSurface,
    Warning,
    WarningSurface,
}
static COLORS: [Color; 18] = [
    Color::Accent,
    Color::OnAccent,
    Color::Background,
    Color::OnBackground,
    Color::SurfaceDim,
    Color::Surface,
    Color::SurfaceBright,
    Color::OnSurface,
    Color::AccentuatedSurfaceDim,
    Color::AccentuatedSurface,
    Color::AccentuatedSurfaceBright,
    Color::Border,
    Color::Success,
    Color::SuccessSurface,
    Color::Destructive,
    Color::DestructiveSurface,
    Color::Warning,
    Color::WarningSurface,
];
impl Color {
    pub fn iter() -> Iter<'static, Color> {
        COLORS.iter()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Accent => "--accent",
            Color::OnAccent => "--on-accent",
            Color::Background => "--background",
            Color::OnBackground => "--on-background",
            Color::SurfaceDim => "--surface-dim",
            Color::Surface => "--surface",
            Color::SurfaceBright => "--surface-bright",
            Color::OnSurface => "--on-surface",
            Color::AccentuatedSurfaceDim => "--accentuated-surface-dim",
            Color::AccentuatedSurface => "--accentuated-surface",
            Color::AccentuatedSurfaceBright => "--accentuated-surface-bright",
            Color::Border => "--border",
            Color::Success => "--success",
            Color::SuccessSurface => "--success-surface",
            Color::Destructive => "--destructive",
            Color::DestructiveSurface => "--destructive-surface",
            Color::Warning => "--warning",
            Color::WarningSurface => "--warning-surface",
        }
    }
}
