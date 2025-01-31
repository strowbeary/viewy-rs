use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};

use dyn_clone::DynClone;

use crate::components::badge::{Badge, BadgeModifiers, BadgeSupport};
use crate::components::icons::IconPack;
use crate::components::{Alignment, Appendable, HStack, Icon, Text, TextStyle};
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};

#[derive(Debug, Clone)]
pub struct Tag {
    node: Node,
    pub label: String,
    pub icon: Option<Box<dyn IconPack>>,
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
    pub fn icon<T>(&mut self, icon: T) -> Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self.clone()
    }

    pub fn warning(&mut self) -> Self {
        self.add_class("tag--warning")
    }

    pub fn destructive(&mut self) -> Self {
        self.add_class("tag--destructive")
    }

    pub fn success(&mut self) -> Self {
        self.add_class("tag--success")
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
            let mut icon = Icon::new(icon).size(16).stroke_width(2);
            tag.node.children.push(icon.render());
        }

        tag.node
            .children
            .push({ Text::new(&self.label, TextStyle::Overline) }.render());

        if let Some(badge) = tag.badge {
            tag.node.children.push(badge.render());
        }

        tag.node
    }
}
