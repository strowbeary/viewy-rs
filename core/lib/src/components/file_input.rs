use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers};
use crate::components::{Appendable, ChildContainer};



#[derive(Debug, Clone)]
pub struct FileInput {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
}

impl NodeContainer for FileInput {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<FileInput> for FileInput {}

impl FileInput {
    pub fn new(name: &str) -> Self {
        FileInput {
            children: vec![],
            node: Node::default(),
        }
            .set_attr("id", &format!("file-input-{}", name))
            .set_attr("name", name)
    }
}


impl Renderable for FileInput {
    fn render(&self) -> Node {

        let mut file_input = self.clone()
            .add_class("file-input")
            .set_attr("type", "file")
            .tag("input");


        let mut node = file_input.node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render()));
        node
    }
}