use std::borrow::BorrowMut;

use crate::components::{Appendable, ChildContainer};
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

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

    pub fn action(&mut self, url: &str) -> Self {
        self.tag("a");
        self.set_attr("href", url);
        self.add_class("clickable");
        self.clone()
    }

    pub fn highlighted(&mut self, is_highlighted: bool) -> Self {
        if is_highlighted {
            self.add_class("card--highlighted");
        } else {
            self.remove_class("card--highlighted");
        }
        self.clone()
    }

    pub fn remove_highlight_on_submit(&mut self, form_name: &str) -> Self {
        self.set_attr("data-remove-highlight-on-submit", form_name)
            .clone()
    }

    pub fn highlight_on_submit(&mut self, form_name: &str) -> Self {
        self.set_attr("data-highlight-on-submit", form_name)
            .clone()
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