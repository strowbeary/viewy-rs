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

    pub fn action(&mut self, url: &str) -> &mut Self {
        self.tag("a").set_attr("href", url).add_class("clickable")
    }

    pub fn highlighted(&mut self, is_highlighted: bool) -> &mut Self {
        if is_highlighted {
            self.add_class("card--highlighted")
        } else {
            self.remove_class("card--highlighted")
        }
    }

    pub fn remove_highlight_on_submit(&mut self, form_name: &str) -> &mut Self {
        self.set_attr("data-remove-highlight-on-submit", form_name)
    }

    pub fn highlight_on_submit(&mut self, form_name: &str) -> &mut Self {
        self.set_attr("data-highlight-on-submit", form_name)
    }


    /// Make the card submit specified form when clicked
    /// ```rust
    ///use viewy::components::CardStyle;
    /// View::new()
    ///    .append_child({
    ///        Form::new("formName", "/")
    ///    })
    ///    .append_child({
    ///        Card::new(CardStyle::Filled)
    ///            .attach_to_form("formName")
    ///        })
    /// ```
    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
        self.add_class("clickable")
            .set_attr("data-submit-form", form_name)
    }
}


impl NodeContainer for Card {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Card {}

impl ChildContainer for Card {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Card {}

impl Renderable for Card {
    fn render(mut self) -> Node {
        let card_style = format!("card--{:?}", self.style).to_lowercase();
        self
            .add_class("card")
            .add_class(&card_style);
        let child_nodes= self.children.into_iter()
            .map(|child| child.render()).collect();
        self.node.children = child_nodes;
        self.node
    }
}