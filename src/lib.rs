#[macro_use]
extern crate view;
extern crate minifier;
extern crate grass;



pub use view::*;
pub mod components;
mod template_compilation_tools;

pub use template_compilation_tools::StyleRegistery;
use crate::template_compilation_tools::ScriptRegistry;

pub trait View {
    fn get_html(&self) -> String;
    fn register_css(&self, style_registery: &mut StyleRegistery);
    fn register_js(&self, script_registery: &mut ScriptRegistry);
}

pub struct Component<S, R: View>(pub fn (S) -> R);

impl<S, R: View>  Component<S, R> {
    pub fn compile(&self, state: S) -> (String, String, String) {
        let mut style_registery = StyleRegistery::new();
        let mut script_registery = ScriptRegistry::new();
        let compiled_view = self.0(state);
        compiled_view.register_css(&mut style_registery);
        compiled_view.register_js(&mut script_registery);
        (compiled_view.get_html(), style_registery.get_css(), script_registery.get_js())
    }
}