use crate::{View, StyleRegistery, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::view::{DefaultModifiers, ViewContainer};
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
    pub view: View,
    pub style: CardStyle,

}
impl ViewContainer for Card {
    fn get_view(&mut self) -> &mut View {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<Card> for Card {}

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Card {
            children: vec![],
            view: View::default(),
            style: style.clone(),
        }
    }
    pub fn add_view_child<'a, T>(&'a mut self, child: Box<T>)
        where
            T: 'static + Renderable,
    {
        self.children.push(child);
    }
}

impl Renderable for Card {
    fn render(&self, style_registery: &mut StyleRegistery, script_registery: &mut ScriptRegistry) -> View {
        style_registery.register_stylesheet(
            "card",
            include_str!("../themes/components/card.scss"),
        );
        let mut view = self.clone()
            .add_class("card")
            .add_class(format!("card--{:?}", self.style).to_lowercase().as_str())
            .view;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render(style_registery, script_registery)));
        view
    }
}