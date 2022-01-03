use std::borrow::{Borrow, BorrowMut};

use crate::{DefaultModifiers, Renderable};
use crate::components::{Alignment, Appendable, Button, HStack, Text, TextStyle, ButtonStyle};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct Snackbar {
    node: Node,
    pub content: String,
    pub action: Option<Button>,
}

impl Snackbar {
    pub fn new<T: AsRef<str>>(title: T) -> Snackbar {
        Snackbar {
            node: Default::default(),
            content: title.as_ref().to_string(),
            action: None
        }
    }
    pub fn set_action_button(&mut self, button: Button) -> Self {
        if matches!(button.style, ButtonStyle::Link) {
            self.action = Some(button);
        } else {
            println!("[viewy-rs] (error) You can only use a button with ButtonStyle::Link style as snackbar action");
        }
        self.clone()
    }
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