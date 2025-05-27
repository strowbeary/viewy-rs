use std::borrow::BorrowMut;
use std::fmt::Debug;


use crate::components::badge::{Badge, BadgeModifiers, BadgeSupport};
use crate::components::icons::IconPack;
use crate::components::{Icon, Text, TextStyle};
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

impl DefaultModifiers for Tag {}

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
    fn render(mut self) -> Node {
        self.add_class("tag");

        if let Some(icon) = self.icon {
            let mut icon = Icon::new(icon);icon.size(16).stroke_width(2);
            self.node.children.push(icon.render());
        }

        self.node
            .children
            .push({ Text::new(&self.label, TextStyle::Overline) }.render());

        if let Some(badge) = self.badge {
            self.node.children.push(badge.render());
        }

        self.node
    }
}
