use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::components::{Appendable, ChildContainer};


/// Used to set card style.
#[derive(Debug, Clone)]
pub enum CardStyle {
    Outlined,
    Filled,
    Raised,
}

/// An outlined view to emphasize a content.
#[derive(Debug, Clone)]
pub struct Card {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub style: CardStyle,
}

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Card {
            children: vec![],
            node: Node::default(),
            style: style.clone(),
        }
    }
}


impl NodeContainer for Card {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Card> for Card {}

impl ChildContainer for Card {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Card {}

impl Renderable for Card {
    fn render(&mut self) -> Node {
        let style = self.style.clone();
        self
            .add_class("card")
            .add_class({
                match style {
                    CardStyle::Outlined => {"card--outlined"}
                    CardStyle::Filled => {"card--filled"}
                    CardStyle::Raised => {"card--raised"}
                }
            });

        let mut rendered_children: Vec<Node> = self.children.iter_mut()
            .map(|child| {
                child.render()
            })
            .collect();
        self.get_node().clone().children.append({
            &mut rendered_children
        });
        self.get_node().clone()
    }
}