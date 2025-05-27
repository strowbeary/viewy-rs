use crate::{Renderable};
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

impl DefaultModifiers for VStack {}

impl VStack {
    pub fn new(alignment: Alignment) -> Self {
        VStack {
            children: vec![],
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn justify_content(&mut self, value: &str) -> &mut Self {
        self.node.node_style.push(("justify-content".to_string(), value.to_string()));
        self
    }
    pub fn flex_wrap(&mut self) -> &mut Self {
        self.node.node_style.push(("flex-wrap".to_string(), "wrap".to_string()));
        self
    }

}
impl ChildContainer for VStack {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        self.children.borrow_mut()
    }
}
impl Appendable for VStack {}
impl Renderable for VStack {
    fn render(mut self) -> Node {
        let alignment = match self.alignment {
            Alignment::Center => {"center"}
            Alignment::Start => {"start"}
            Alignment::End => {"end"}
            Alignment::Stretch => {"stretch"}
        };
        self.add_class("stack")
            .add_class("stack--vertical")
            .add_class(
                &format!("stack--align-{alignment}", )
            );
        self.children.into_iter()
            .for_each(|child|
                self.node.children.push(child.render()));
        self.node
    }
}