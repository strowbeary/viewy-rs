use crate::{View, StyleRegistery, Renderable};
use std::sync::Mutex;
use crate::component_style;
use crate::template_compilation_tools::ScriptRegistry;

#[derive(Clone, Debug)]
pub enum CardStyle {
    Outlined,
    Filled,
    Raised,
}

pub struct Card {
    pub style: CardStyle,
    pub class_list: Vec<String>,
    pub children: Vec<View>,
}

impl Card {
    pub fn new(style: CardStyle) -> Self {
        Card {
            style: style.clone(),
            class_list: vec!["card".to_string()],
            children: vec![]
        }
    }
    pub fn add_view_child(&mut self, child: View) {
        self.children.push(child);
    }
}

impl Renderable for Card {
    fn get_html(&self) -> String {
        let classes = self.class_list.join(" ");
        let content: Vec<String> = self.children.iter()
            .map(|child| child.get_html())
            .collect();
        let style_class = format!("card--{:?}", self.style).to_lowercase();
        format!(
            "<div class=\"{} {}\">{}</div>",
            classes,
            style_class,
            content.join("")
        )
    }

    fn register_css(&self, style_registery: &mut StyleRegistery) {
        self.children.iter()
            .for_each(|child| child.register_css(style_registery));
        style_registery.register_stylesheet("card", component_style!("card"));
    }

    fn register_js(&self, script_registery: &mut ScriptRegistry) {
        self.children.iter()
            .for_each(|child| child.register_js(script_registery));
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            style: CardStyle::Outlined,
            class_list: vec!["card".to_string()],
            children: vec![]
        }
    }
}