use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};

#[derive(Clone, Debug)]
pub enum BadgeType {
    Important,
    Normal
}

#[derive(Clone, Debug)]
pub struct Badge {
    pub node: Node,
    pub value: Option<String>,
    pub badge_type: BadgeType
}

impl Badge {
    pub fn new(badge_type: BadgeType, value: &str) -> Self {
        Badge {
            node: Default::default(),
            value: Some(value.to_string()),
            badge_type
        }
    }

    pub fn textless(badge_type: BadgeType) -> Self {
        Badge {
            node: Default::default(),
            value: None,
            badge_type
        }
    }

    pub fn remove_on_click(&mut self, id: &str) -> Self {
        self.set_attr("data-remove-on-click", id)
            .clone()
    }
}

impl NodeContainer for Badge {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Badge> for Badge {}

impl Renderable for Badge {
    fn render(&self) -> Node {
        let mut badge = self.clone()
            .add_class("badge");
        match &self.value {
            None => badge.add_class("badge--textless"),
            Some(_) => badge.add_class("badge--with-text"),
        };
        match &self.badge_type {
            BadgeType::Important => badge.add_class("badge--important"),
            BadgeType::Normal => badge.add_class("badge--normal")
        };
        badge.node.text = self.value.clone();
        badge.node
    }
}

pub trait BadgeSupport {
    fn add_badge(&mut self, badge: Badge);
}

pub trait BadgeModifiers: Sized + Clone + BadgeSupport {
    fn badge(&mut self, count: &usize) -> Self {
        if count.gt(&99) {
            self.add_badge(Badge::new(BadgeType::Important, "99+"));
        } else {
            self.add_badge(Badge::new(BadgeType::Important,&count.to_string()));
        }
        self.clone()
    }
}