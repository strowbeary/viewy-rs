use crate::theme::ThemeLoader;

fn get_stylesheets() -> Vec<String> {
    let theme = ThemeLoader::load_from_config_folder();

    vec![
        format!(
            "$primary: {primary};\
            $destructive: {destructive};",
            primary = theme.colors.accent.light,
            destructive = theme.colors.destructive.light
        ),
        format!(
            "$border-radius: {border_radius};",
            border_radius = theme.shapes.border_radius
        ),
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
        include_str!("../themes/components/textfield.scss").to_string(),
        include_str!("../themes/components/titlebar.scss").to_string(),
        include_str!("../themes/components/menu.scss").to_string(),
        include_str!("../themes/components/table.scss").to_string(),
        include_str!("../themes/components/checkbox.scss").to_string(),
        include_str!("../themes/components/avatar.scss").to_string(),
        include_str!("../themes/components/tag.scss").to_string(),
    ]
}

fn get_scripts() -> Vec<String> {
    vec![
        include_str!("../js/async-form.js").to_string(),
        include_str!("../js/menu.js").to_string(),
        include_str!("../js/popover.js").to_string(),
        include_str!("../js/table.js").to_string(),
    ]
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
        print!("Compiling theme");
        let theme = Self {
            script: Assets::compile_scripts(),
            stylesheet: Assets::compile_theme(),
        };

        println!(" [Done]");
        theme
    }
    fn compile_theme() -> String {
        match grass::from_string(
            get_stylesheets().join(""),
            &grass::Options::default(),
        ) {
            Ok(css) => minifier::css::minify(css.as_str()).unwrap(),
            Err(err) => {
                println!("{:?}", err);
                String::new()
            }
        }
    }
    fn compile_scripts() -> String {
        let joined_scripts: String = get_scripts().join("");
        minifier::js::minify(joined_scripts.as_str())
    }
}
