use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Renderable};
use crate::components::{Alignment, Appendable, Button, HStack, Text, TextStyle};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct Snackbar {
    node: Node,
    pub content: String,
    pub action: Option<Button>,
}

impl NodeContainer for Snackbar {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Snackbar> for Snackbar {}

impl Renderable for Snackbar {
    fn render(&self) -> Node {
        let mut snackbar = self.clone()
            .add_class("snackbar");
        snackbar.node.children.push({
            let mut content = HStack::new(Alignment::Center)
                .append_child({
                    Text::new(&self.content, TextStyle::Body)
                });
            if let Some(action) = self.action.clone() {
                content.append_child(action);
            }
            content
        }.render());


        snackbar.node
    }
}