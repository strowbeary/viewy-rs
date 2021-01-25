use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistery};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
    Start,
    End,
    Stretch
}

#[derive(Debug, Clone)]
pub struct VStack {
    children: Vec<Box<dyn Renderable>>,
    pub view: Node,
    pub alignment: Alignment,
}
impl NodeContainer for VStack {
    fn get_node(&mut self) -> &mut Node {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<VStack> for VStack {}

impl VStack {
    pub fn new(alignment: Alignment) -> Self {
        VStack {
            children: vec![],
            view: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.view.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self.clone()
    }
    pub fn justify_content(&mut self, value: &str) -> Self {
        self.view.node_style.push(("justify-content".to_string(), value.to_string()));
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
    fn render(&self, style_registery: &mut StyleRegistery, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "stack",
            include_str!("../themes/components/stack.scss"),
        );
        let mut view = self
            .clone()
            .add_class("stack")
            .add_class("stack--vertical")
            .add_class(
                format!("stack--align-{:?}", self.alignment)
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