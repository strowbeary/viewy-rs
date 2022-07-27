use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};

use dyn_clone::DynClone;

use crate::{DefaultModifiers, Renderable};
use crate::components::{Alignment, Appendable, HStack, Icon, Text, TextStyle};
use crate::components::badge::{Badge, BadgeSupport, BadgeModifiers};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct Tag {
    node: Node,
    pub label: String,
    pub icon: Option<String>,
    pub badge: Option<Badge>,
}


impl Tag {
    pub fn new(label: &str) -> Self {
        Self {
            node: Node::default(),
            label: label.to_string(),
            icon: None,
            badge: None,
        }
    }

    /// Set tag's icon
    pub fn icon(&mut self, name: &str) -> Self {
        self.icon = Some(name.to_string());
        self.clone()
    }

}

impl DefaultModifiers<Tag> for Tag {}


impl BadgeSupport for Tag {
    fn add_badge(&mut self, badge: Badge) {
        self.badge = Some(badge);
    }
}

impl BadgeModifiers for Tag {}

impl NodeContainer for Tag {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl Renderable for Tag {
    fn render(&self) -> Node {
        let mut tag = self.clone();
        tag.get_node().class_list.insert("tag".to_string());

        if let Some(icon) = tag.icon {
            let mut icon = Icon::new(icon.as_str())
                .size(16)
                .stroke_width(2);
            tag.node.children.push(icon.render());
        }

        tag.node.children.push({
            Text::new(&self.label, TextStyle::Overline)
        }.render());

        if let Some(badge) = tag.badge {
            tag.node.children.push(badge.render());
        }

        tag.node
    }
}