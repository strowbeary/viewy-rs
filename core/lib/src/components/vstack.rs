use crate::renderer::{Renderable, ToHtml, StyleRegistry, ScriptRegistry};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp};

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
    node: Node,
    pub alignment: Alignment,
}

impl NodeContainer for VStack {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<VStack> for VStack {}


impl ToHtml for VStack {}

impl VStack {
    pub fn new(alignment: Alignment) -> Self {
        VStack {
            children: vec![],
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self.clone()
    }
    pub fn justify_content(&mut self, value: &str) -> Self {
        self.node.node_style.push(("justify-content".to_string(), value.to_string()));
        self.clone()
    }
    pub fn append_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }

}

impl Renderable for VStack {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "stack",
            include_str!("../themes/components/stack.scss"),
        );
        let mut stack = self
            .clone()
            .add_class("stack")
            .add_class("stack--vertical")
            .add_class(
                format!("stack--align-{:?}", self.alignment)
                    .to_lowercase()
                    .as_str()
            );
        self.children.iter()
            .for_each(|child|
                stack.node.children.push(child.render(style_registery, script_registery)));
        stack.node
    }
}