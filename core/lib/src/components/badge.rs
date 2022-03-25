use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};

#[derive(Clone, Debug)]
pub struct Badge {
    pub node: Node,
    pub value: String
}

impl Badge {
    pub fn new(value: &str) -> Self {
        Badge {
            node: Default::default(),
            value: value.to_string()
        }
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
        badge.node.text = Some(self.value.clone());
        badge.node
    }
}

pub trait BadgeSupport {
    fn add_badge(&mut self, badge: Badge);
}

pub trait BadgeModifiers: Sized + Clone + BadgeSupport {

    fn badge(&mut self, count: &usize) -> Self {
        if count.gt(&99) {
            self.add_badge(Badge::new("99+"));
        } else {
            self.add_badge(Badge::new(&count.to_string()));
        }
        self.clone()
    }

}