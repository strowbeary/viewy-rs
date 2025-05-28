use crate::node::Node;
use crate::{DefaultModifiers, Renderable};
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub enum BadgeType {
    Important,
    Normal,
}

#[derive(Clone, Debug)]
pub struct Badge {
    pub node: Node,
    pub value: Option<String>,
    pub badge_type: BadgeType,
}

impl Badge {
    pub fn new(badge_type: BadgeType, value: &str) -> Self {
        Badge {
            node: Default::default(),
            value: Some(value.to_string()),
            badge_type,
        }
    }

    pub fn textless(badge_type: BadgeType) -> Self {
        Badge {
            node: Default::default(),
            value: None,
            badge_type,
        }
    }

    pub fn remove_on_click(&mut self, id: &str) -> Self {
        self.set_attr("data-remove-on-click", id).clone()
    }
}

impl Deref for Badge {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl DerefMut for Badge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for Badge {}

impl Renderable for Badge {
    fn render(mut self) -> Node {
        self.add_class("badge");
        match &self.value {
            None => self.add_class("badge--textless"),
            Some(_) => self.add_class("badge--with-text"),
        };
        match &self.badge_type {
            BadgeType::Important => self.add_class("badge--important"),
            BadgeType::Normal => self.add_class("badge--normal"),
        };
        self.node.text = self.value.clone();
        self.node
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
            self.add_badge(Badge::new(BadgeType::Important, &count.to_string()));
        }
        self.clone()
    }
}
