use crate::{StyleRegistry, Renderable};
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{DefaultModifiers, NodeContainer, Node};
use std::borrow::BorrowMut;

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

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Card {
            children: vec![],
            node: Node::default(),
            style: style.clone(),
        }
    }
    pub fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }
}

impl Renderable for Card {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "card",
            include_str!("../themes/components/card.scss"),
        );
        let mut view = self.clone()
            .add_class("card")
            .add_class(format!("card--{:?}", self.style).to_lowercase().as_str())
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render(style_registery, script_registery)));
        view
    }
}