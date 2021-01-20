extern crate minifier;
extern crate grass;

pub mod components;
mod template_compilation_tools;
mod helper_fn;
use components::View;
pub use template_compilation_tools::StyleRegistery;
use crate::template_compilation_tools::ScriptRegistry;

pub trait Renderable {
    fn get_html(&self) -> String;
    fn register_css(&self, style_registery: &mut StyleRegistery);
    fn register_js(&self, script_registery: &mut ScriptRegistry);
}


pub struct Component<S>(pub fn (S) -> View);

impl<S>  Component<S> {
    pub fn compile(&self, state: S) -> (String, String, String) {
        let mut style_registery = StyleRegistery::new();
        let mut script_registery = ScriptRegistry::new();
        let compiled_view = self.0(state);
        compiled_view.register_css(&mut style_registery);
        compiled_view.register_js(&mut script_registery);
        (compiled_view.get_html(), style_registery.get_css(), script_registery.get_js())
    }
}