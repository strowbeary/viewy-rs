use crate::components::Alignment;
use crate::components::{Appendable, ChildContainer};
use crate::node::{Node, NodeContainer};
use crate::{sp, DefaultModifiers, Renderable};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct Grid {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub alignment: Alignment,
}

impl NodeContainer for Grid {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Grid {}

impl Grid {
    pub fn new(alignment: Alignment) -> Self {
        Grid {
            children: vec![],
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self{
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node
            .node_style
            .push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn columns(&mut self, schema: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-template-columns".to_string(), schema.to_string()));
        self
    }

    //pub fn responsive_columns(&mut self, schemas: Vec<(&str, &str)>) -> -> &mut Self{}

    pub fn areas(&mut self, schema: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-template-areas".to_string(), schema.to_string()));
        self
    }

    pub fn rows(&mut self, schema: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-template-rows".to_string(), schema.to_string()));
        self
    }
    pub fn auto_columns(&mut self, size: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-auto-columns".to_string(), size.to_string()));
        self
    }
    pub fn auto_rows(&mut self, size: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-auto-rows".to_string(), size.to_string()));
        self
    }
    pub fn auto_flow(&mut self, direction: &str) -> &mut Self{
        self.node
            .node_style
            .push(("grid-auto-flow".to_string(), direction.to_string()));
        self
    }
    pub fn align_items(&mut self, value: &str) -> &mut Self{
        self.node
            .node_style
            .push(("align-items".to_string(), value.to_string()));
        self
    }
    pub fn justify_content(&mut self, value: &str) -> &mut Self{
        self.node
            .node_style
            .push(("justify-content".to_string(), value.to_string()));
        self
    }
    pub fn justify_items(&mut self, value: &str) -> &mut Self{
        self.node
            .node_style
            .push(("justify-items".to_string(), value.to_string()));
        self
    }
}

impl ChildContainer for Grid {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        self.children.borrow_mut()
    }
}

impl Appendable for Grid {}

impl Renderable for Grid {
    fn render(mut self) -> Node {
        self
            .add_class("grid")
            .add_class(
                format!("grid--align-{:?}", self.alignment)
                    .to_lowercase()
                    .as_str(),
            );
        self.children
            .into_iter()
            .for_each(|child| self.node.children.push(child.render()));
        self.node
    }
}
