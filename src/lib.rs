extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;

mod template_compilation_tools;

pub mod components;
pub mod helper_fn;
pub mod node;

use template_compilation_tools::StyleRegistry;
use template_compilation_tools::ScriptRegistry;
use std::fmt::Debug;
use crate::node::{Node, DefaultModifiers};

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

pub fn compile_page((content, style, script): (String, String, String), theme_variant: &str) -> String {
    format!(r"
        <!doctype html>
        <html>
        <head>
            <title>Viewy-rs showcase</title>
            <style>{style}</style>
            <script>{script}</script>
            <meta charset='utf8' />
        </head>
        <body class='app-theme--{theme_variant}'>
            {content}
        </body>
        </html>
    ",
        style = style,
        script = script,
        content = content,
        theme_variant = theme_variant
    )
}

pub struct Component<S, T: Renderable>(pub fn(S) -> T);

impl<S, T: Renderable> Component<S, T> {
    pub fn compile(&self, state: S) -> (String, String, String) {
        let mut style_registery = StyleRegistry::new();
        let mut script_registery = ScriptRegistry::new();
        let mut compiled_view = self.0(state);
        (compiled_view.render(&mut style_registery, &mut script_registery).get_html(), style_registery.get_css(), script_registery.get_js())
    }
}
