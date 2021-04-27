#![feature(in_band_lifetimes)]
extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;
extern crate dyn_clone;

mod helper_fn;
mod node;
mod modifiers;
mod renderer;
pub mod page;
pub mod components;

pub use modifiers::{DefaultModifiers, Overflow};
pub use renderer::{Renderable};
pub use helper_fn::*;

fn get_stylesheets() -> Vec<String> {
    vec![
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
        include_str!("themes/components/table.scss").to_string(),
        include_str!("themes/components/checkbox.scss").to_string(),
    ]
}

fn get_scripts() -> Vec<String> {
    vec![
        include_str!("js/async-form.js").to_string(),
        include_str!("js/button.js").to_string(),
        include_str!("js/popover.js").to_string(),
        include_str!("js/table.js").to_string(),
    ]
}

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

