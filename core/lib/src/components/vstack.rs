use crate::renderer::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp};
use crate::components::{Appendable, ChildContainer};


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

}
impl ChildContainer for VStack {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for VStack {}
impl Renderable for VStack {
    fn render(&self) -> Node {
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
                stack.node.children.push(child.render()));
        stack.node
    }
}