use std::collections::HashMap;
use grass;
use std::path::Path;
use grass::OutputStyle;
use std::env;

#[derive(Clone)]
pub struct StyleRegistery {
    styles: HashMap<String, String>
}

impl StyleRegistery {
    pub fn new() -> Self {
        StyleRegistery {
            styles: Default::default()
        }
    }
    pub fn register_stylesheet(&mut self, component_name: &str, stylesheet: &str) {
        self.styles.insert(component_name.to_string(), stylesheet.to_string());
    }

    pub fn get_css(&self) -> String {
        let mut stylesheets = vec![
            include_str!("themes/palette.scss").to_string(),
            include_str!("themes/sizing.scss").to_string(),
            include_str!("themes/typography.scss").to_string(),
            include_str!("themes/commons.scss").to_string(),
            include_str!("themes/view.scss").to_string()
        ];
        self.styles.values().for_each(|stylesheet| stylesheets.push(stylesheet.into()));
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
}


#[derive(Clone)]
pub struct ScriptRegistry {
    scripts: HashMap<String, String>
}

impl ScriptRegistry {
    pub fn new() -> Self {
        ScriptRegistry {
            scripts: Default::default()
        }
    }
    pub fn register_script(&mut self, component_name: &str, script: &str) {
        self.scripts.insert(component_name.to_string(), script.to_string());
    }

    pub fn get_js(&self) -> String {
        let scripts: Vec<String> = self.scripts.values()
            .map(|script| script.into()).collect();
        let joined_scripts: String = scripts.join("");
        minifier::js::minify(joined_scripts.as_str())
    }
}