use grass::OutputStyle;
use crate::CONFIG;
use crate::theme::{Config, ConfigLoader};

fn get_stylesheets(config: &Config) -> Vec<String> {
    let mut styles = vec![
        format!("
@use \"sass:color\";
@use \"sass:math\";

$border-radius: {border_radius};

$accent-light: {accent_light};
$accent-dark: {accent_dark};
$on-accent: {on_accent};
$destructive-light: {destructive_light};
$destructive-dark: {destructive_dark};
$on-destructive-light: {on_destructive_light};
$on-destructive-dark: {on_destructive_dark};
$background-light: {background_light};
$background-dark: {background_dark};
$surface-light: {surface_light};
$surface-dark: {surface_dark};

$success-light: {success_light};
$success-dark: {success_dark};
$on-success-light: {on_success_light};
$on-success-dark: {on_success_dark};
            ",
                accent_light = config.colors.accent.light,
                accent_dark = config.colors.accent.dark,
                destructive_light = config.colors.destructive.light,
                on_destructive_light = config.colors.on_destructive.light,
                destructive_dark = config.colors.destructive.dark,
                on_destructive_dark = config.colors.on_destructive.dark,
                success_light = config.colors.success.light,
                on_success_light = config.colors.on_success.light,
                success_dark = config.colors.success.dark,
                on_success_dark = config.colors.on_success.dark,
                on_accent = config.colors.on_accent.light,
                border_radius = config.shapes.border_radius,
                background_light = config.colors.background.light,
                background_dark = config.colors.background.dark,
                surface_light = config.colors.surface.light,
                surface_dark = config.colors.surface.dark
        ),
        include_str!("../themes/luminance.scss").to_string(),
        include_str!("../themes/palette.scss").to_string(),
        include_str!("../themes/sizing.scss").to_string(),
        include_str!("../themes/typography.scss").to_string(),
        include_str!("../themes/commons.scss").to_string(),
        include_str!("../themes/view.scss").to_string(),
        include_str!("../themes/components/button.scss").to_string(),
        include_str!("../themes/components/card.scss").to_string(),
        include_str!("../themes/components/divider.scss").to_string(),
        include_str!("../themes/components/form.scss").to_string(),
        include_str!("../themes/components/grid.scss").to_string(),
        include_str!("../themes/components/image.scss").to_string(),
        include_str!("../themes/components/picker.scss").to_string(),
        include_str!("../themes/components/popover.scss").to_string(),
        include_str!("../themes/components/stack.scss").to_string(),
        include_str!("../themes/components/text.scss").to_string(),
        include_str!("../themes/components/field.scss").to_string(),
        include_str!("../themes/components/titlebar.scss").to_string(),
        include_str!("../themes/components/menu.scss").to_string(),
        include_str!("../themes/components/table.scss").to_string(),
        include_str!("../themes/components/checkbox.scss").to_string(),
        include_str!("../themes/components/avatar.scss").to_string(),
        include_str!("../themes/components/tag.scss").to_string(),
        include_str!("../themes/components/popup.scss").to_string(),
        include_str!("../themes/components/file-input.scss").to_string(),
        include_str!("../themes/components/multiple-file-input.scss").to_string(),
        include_str!("../themes/components/color-picker.scss").to_string(),
        include_str!("../themes/components/signature-field.scss").to_string(),
        include_str!("../themes/components/snackbar.scss").to_string(),
        include_str!("../themes/components/tabs.scss").to_string(),
        include_str!("../themes/components/badge.scss").to_string(),
        include_str!("../themes/components/gauge.scss").to_string(),
        include_str!("../themes/components/disclosure.scss").to_string(),
        include_str!("../themes/components/table-of-contents.scss").to_string(),
        include_str!("../themes/components/progress-bar.scss").to_string(),
        include_str!("../themes/print.scss").to_string(),
    ];
    if config.features.rich_text_editor {
        styles.push(include_str!("../themes/quill.scss").to_string());
    }
    if config.features.sortable_stack {
        styles.push(include_str!("../themes/components/sortable-stack.scss").to_string());
    }

    styles
}

fn get_scripts(config: &Config) -> Vec<String> {

    let mut scripts = vec![
        include_str!("../js/index.js").to_string(),
        include_str!("../js/popper.js").to_string(),
        include_str!("../js/form.js").to_string(),
        include_str!("../js/menu.js").to_string(),
        include_str!("../js/popover.js").to_string(),
        include_str!("../js/table.js").to_string(),
        include_str!("../js/popup.js").to_string(),
        include_str!("../js/file-input.js").to_string(),
        include_str!("../js/searchable-input.js").to_string(),
        include_str!("../js/signature-field.js").to_string(),
        include_str!("../js/tabs.js").to_string(),
        include_str!("../js/snackbar.js").to_string(),
        include_str!("../js/dynamic_content.js").to_string(),
        include_str!("../js/picker.js").to_string(),
        include_str!("../js/duration-field.js").to_string(),
        include_str!("../js/highlight-control.js").to_string(),
        include_str!("../js/table-of-contents.js").to_string(),
        include_str!("../js/multiple-file-input.js").to_string(),
    ];
    if config.features.rich_text_editor {
        scripts.push(include_str!("../js/quill-editor.js").to_string());
    }
    if config.features.sortable_stack {
        scripts.push(include_str!("../js/Sortable.min.js").to_string());
        scripts.push(include_str!("../js/sortable-stack.js").to_string());
    }
    scripts
}


/// At startup it compile theme and minify js sources.
/// You have to use it to serve these assets to the client.
///
/// **How to use**
///
/// You have to add these routes to your project
/// ```rust
/// use viewy::engine::Assets;
/// #[get("/app.css")]
/// fn get_stylesheet(assets: rocket::State<Assets>) -> Css<String> {
///     Css(assets.inner().clone().stylesheet)
/// }
///
/// #[get("/app.js")]
/// fn get_scripts(assets: rocket::State<Assets>) -> JavaScript<String> {
///     JavaScript(assets.inner().clone().script)
/// }
/// ```
/// And add an `Assets` instance to rocket state
/// ```rust
/// rocket::ignite()
///     .manage(rocket::Assets::new())
/// ```
#[derive(Clone)]
pub struct Assets {
    pub script: String,
    pub stylesheet: String,
}

impl Assets {
    pub fn new() -> Self {
        print!("Load config");
        let config = &*CONFIG;
        let theme = Self {
            script: Assets::compile_scripts(config),
            stylesheet: Assets::compile_theme(config),
        };

        println!(" [Done]");
        theme
    }
    fn compile_theme(config: &Config) -> String {
        println!("Compile theme ");
        let stylesheets = get_stylesheets(&config).join("");
        match grass::from_string(
            stylesheets,
            &grass::Options::default().style(OutputStyle::Compressed),
        ) {
            Ok(css) => {
                println!(" [Done]");
                css
            }
            Err(err) => {
                println!(" [Error]");
                println!("{:?}", err);
                get_stylesheets(config).join("")
            }
        }
    }
    fn compile_scripts(config: &Config) -> String {
        let joined_scripts: String = get_scripts(config).join("");
        minifier::js::minify(joined_scripts.as_str()).to_string();
        joined_scripts
    }
}
