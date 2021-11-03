use std::borrow::BorrowMut;

use crate::Renderable;
use crate::DefaultModifiers;
use crate::components::{Appendable, ChildContainer};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct FileInput {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    auto_submit: bool,
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
            auto_submit: false,
        }
            .set_attr("id", &format!("file-input-{}", name))
            .set_attr("name", name)
    }

    pub fn accept(&mut self, mime_types: &str) -> Self {
        self.set_attr("accept", mime_types)
    }

    pub fn multiple(&mut self) -> Self {
        self.set_attr("multiple", "multiple")
    }

    pub fn auto_submit(&mut self, is_auto_submit: bool) -> Self {
        if is_auto_submit {
            self.auto_submit = is_auto_submit;
        }
        self.clone()
    }
}


impl Renderable for FileInput {
    fn render(&self) -> Node {
        let mut file_input = self.clone()
            .add_class("file-input")
            .set_attr("type", "file")
            .tag("input");

        if self.auto_submit {
            file_input.set_attr("data-auto-submit", &self.auto_submit.to_string());
        }

        let mut node = file_input.node;

        self.children.iter()
            .for_each(|child| {
                node.children.push(child.render());
            });
        node
    }
}