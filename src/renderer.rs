use std::fmt::Debug;
use std::collections::HashMap;
use grass;
use crate::node::Node;

#[derive(Clone)]
pub struct StyleRegistry {
    styles: HashMap<String, String>
}

impl StyleRegistry {
    pub fn new() -> Self {
        StyleRegistry {
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

pub trait Renderable: Debug + RenderableClone {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node;
}

pub trait RenderableClone {
    fn clone_box(&self) -> Box<dyn Renderable>;
}

impl<T> RenderableClone for T
    where
        T: 'static + Renderable + Clone,
{
    fn clone_box(&self) -> Box<dyn Renderable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Renderable> {
    fn clone(&self) -> Box<dyn Renderable> {
        self.clone_box()
    }
}

pub trait ToHtml : Renderable {
    fn compile(&self) -> (String, String, String) {
        let mut style_registery = StyleRegistry::new();
        let mut script_registery = ScriptRegistry::new();
        let root_node: Node = self.render(&mut style_registery, &mut script_registery);
        let popovers_html: Vec<String> = root_node.get_popovers().iter()
            .map(|popover| popover.render(&mut style_registery, &mut script_registery))
            .map(|node| node.get_html())
            .collect();
        (
            format!("{view} {popover}", view = root_node.get_html(), popover = popovers_html.join("")),
            style_registery.get_css(),
            script_registery.get_js()
        )
    }
}