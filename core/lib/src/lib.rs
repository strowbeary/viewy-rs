extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;

mod helper_fn;
mod node;
mod modifiers;
mod renderer;
pub mod components;
pub mod component;

pub use modifiers::DefaultModifiers;
pub use renderer::ToHtml;
pub use helper_fn::*;

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
        let mut stylesheets = vec![
            include_str!("themes/palette.scss").to_string(),
            include_str!("themes/sizing.scss").to_string(),
            include_str!("themes/typography.scss").to_string(),
            include_str!("themes/commons.scss").to_string(),
            include_str!("themes/view.scss").to_string(),
            include_str!("themes/components/button.scss").to_string(),
            include_str!("themes/components/card.scss").to_string(),
            include_str!("themes/components/divider.scss").to_string(),
            include_str!("themes/components/form.scss").to_string(),
            include_str!("themes/components/grid.scss").to_string(),
            include_str!("themes/components/image.scss").to_string(),
            include_str!("themes/components/picker.scss").to_string(),
            include_str!("themes/components/popover.scss").to_string(),
            include_str!("themes/components/stack.scss").to_string(),
            include_str!("themes/components/text.scss").to_string(),
            include_str!("themes/components/textfield.scss").to_string(),
            include_str!("themes/components/titlebar.scss").to_string(),
            include_str!("themes/components/menu.scss").to_string(),
        ];
        match grass::from_string(
            stylesheets.join(""),
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
        let mut scripts = vec![
            include_str!("js/async-form.js").to_string(),
            include_str!("js/button.js").to_string(),
            include_str!("js/picker-segmented.js").to_string(),
            include_str!("js/popover.js").to_string(),
        ];
        let joined_scripts: String = scripts.join("");
        minifier::js::minify(joined_scripts.as_str())
    }
}


pub fn compile_page(title: String, content: String, theme_variant: &str) -> String {
    format!(r"
        <!doctype html>
        <html>
        <head>
            <title>{title}</title>
            <link href='/app.css' rel='stylesheet'>
            <script src='https://unpkg.com/@popperjs/core@2'></script>
            <script src='/app.js'></script>
            <meta charset='utf8' />
        </head>
        <body class='app-themes--{theme_variant}'>
            {content}
        </body>
        </html>
    ",
            title = title,
            content = content,
            theme_variant = theme_variant
    )
}

