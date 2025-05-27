use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::{DefaultModifiers, Renderable, scale};
use crate::components::{Alignment, Appendable, Button, ButtonStyle, HStack, Text, TextStyle};
use crate::components::icons::Lucide;
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub enum SnackbarType {
    Error,
    Success,
    Neutral,
}

#[derive(Debug, Clone)]
pub struct Snackbar {
    node: Node,
    pub content: String,
    pub action: Option<Button>,
    pub snackbar_type: SnackbarType,
}

impl Snackbar {
    pub fn new<T: AsRef<str>>(snackbar_type: SnackbarType, title: T) -> Snackbar {
        Snackbar {
            node: Default::default(),
            content: title.as_ref().to_string(),
            action: None,
            snackbar_type,
        }
    }

    pub fn closable(&mut self) -> &mut Self {
        let id = Uuid::new_v4();
        self.set_attr("data-snackbar-id", &id.to_string());
        self.action = Some({
            let mut btn = Button::icon_only(Lucide::X, ButtonStyle::Flat);
            btn.set_attr("data-snackbar-closing-button", &id.to_string());
            btn
        });
        self
    }

    pub fn set_action_button(&mut self, button: Button) -> &mut Self {
        if matches!(button.style, ButtonStyle::Link) {
            self.action = Some(button);
        } else {
            println!("[viewy-rs] (error) You can only use a button with ButtonStyle::Link style as snackbar action");
        }
        self
    }
}

impl NodeContainer for Snackbar {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Snackbar {}

impl Renderable for Snackbar {
    fn render(mut self) -> Node {
        self.add_class("snackbar");

        let snackbar_type = match self.snackbar_type {
            SnackbarType::Error => "snackbar--error",
            SnackbarType::Success => "snackbar--success",
            SnackbarType::Neutral => "snackbar--neutral",
        };
        self.add_class(snackbar_type);

        self.node.children.push({
            let mut content = HStack::new(Alignment::Center);
            content.gap(vec![scale(3)])
                .append_child({
                    Text::new(&self.content, TextStyle::Body)
                });
            if let Some(action) = self.action {
                content.append_child(action);
            }
            content
        }.render());


        self.node
    }
}