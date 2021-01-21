use crate::view::{View, DefaultModifiers, ViewContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistery};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum HorizontalAlignment {
    Center,
    Leading,
    Trailing,
}

#[derive(Debug, Clone)]
pub struct VStack {
    children: Vec<Box<dyn Renderable>>,
    pub view: View,
    pub alignment: HorizontalAlignment,
}
impl ViewContainer for VStack {
    fn get_view(&mut self) -> &mut View {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<VStack> for VStack {}

impl VStack {
    pub fn new(alignment: HorizontalAlignment) -> Self {
        VStack {
            children: vec![],
            view: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, value: i32) -> Self {
        self.view.view_style.push(("gap".to_string(), sp(value)));
        self.clone()
    }
    pub fn add_view_child<'a, T>(&'a mut self, child: Box<T>)
        where
            T: 'static + Renderable,
    {
        self.children.push(child);
    }

}

impl Renderable for VStack {
    fn render(&self, style_registery: &mut StyleRegistery, script_registery: &mut ScriptRegistry) -> View {
        style_registery.register_stylesheet(
            "stack",
            include_str!("../themes/components/stack.scss"),
        );
        let mut view = self
            .clone()
            .add_class("stack")
            .add_class("stack--vertical")
            .add_class(
                format!("stack--vertical--align-{:?}", self.alignment)
                    .to_lowercase()
                    .as_str()
            )
            .view;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render(style_registery, script_registery)));
        view
    }
}