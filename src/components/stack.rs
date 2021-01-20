use crate::view::{View, DefaultModifiers};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistery};
use crate::template_compilation_tools::ScriptRegistry;

#[derive(Debug, Clone)]
pub enum HorizontalAlignment {
    Center,
    Leading,
    Trailing
}
#[derive(Debug, Clone)]
pub struct VStack {
    view: View,
    alignment: HorizontalAlignment
}

impl DefaultModifiers<VStack> for VStack {}

impl VStack {
    pub fn new(alignment: HorizontalAlignment) -> Self{
       VStack {
           view: Default::default(),
           alignment: HorizontalAlignment::Center
       }
           .add_class("stack")
           .add_class("stack--vertical")
           .add_class(format!("stack--vertical--{:?}", alignment).to_lowercase().as_str())
    }
    pub fn gap(&mut self, value: i32) -> Self {
        self.view.view_style.push(("gap".to_string(), sp(value)));
        self.clone()
    }
}

impl Renderable for VStack {
    fn register_css(&self, style_registery: &mut StyleRegistery) {
        style_registery.register_stylesheet("stack", include_str!("../themes/components/stack.scss"));
    }

    fn register_js(&self, script_registery: &mut ScriptRegistry) {

        self.view.children.iter()
            .for_each(|child| child.register_js(script_registery));
    }
}