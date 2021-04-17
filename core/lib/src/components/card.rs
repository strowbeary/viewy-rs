use crate::renderer::{Renderable, ToHtml};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::component::{Appendable, ChildContainer};

#[derive(Debug, Clone)]
pub enum CardStyle {
    Outlined,
    Filled,
    Raised,
}

#[derive(Debug, Clone)]
pub struct Card {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub style: CardStyle,

}
impl NodeContainer for Card {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Card> for Card {}
impl ToHtml for Card {}



impl Card {
    pub fn new(style: CardStyle) -> Self {
        Card {
            children: vec![],
            node: Node::default(),
            style: style.clone(),
        }
    }
}
impl ChildContainer for Card {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Card {}

impl Renderable for Card {
    fn render(&self) -> Node {
        let mut view = self.clone()
            .add_class("card")
            .add_class(format!("card--{:?}", self.style).to_lowercase().as_str())
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view
    }
}