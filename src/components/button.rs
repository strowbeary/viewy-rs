use crate::{StyleRegistry, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{Node, DefaultModifiers, NodeContainer};
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle};

#[derive(Debug, Clone)]
pub enum ButtonStyle {
    Link,
    Flat,
    Outlined,
    Filled,
}

#[derive(Debug, Clone)]
pub struct Button {
    children: Vec<Box<dyn Renderable>>,
    pub view: Node,
    pub label: String,
    pub style: ButtonStyle,
}

impl NodeContainer for Button {
    fn get_node(&mut self) -> &mut Node {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<Button> for Button {}

impl Button {
    pub fn new(label: &str, style: ButtonStyle) -> Self {
        Button {
            children: vec![],
            view: Node::default(),
            label: label.to_string(),
            style,
        }
    }
    pub fn destructive(&mut self) -> Self {
        self.add_class(format!("button--{:?}--destructive", self.style).to_lowercase().as_str())
    }
    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        if is_disabled {
            self.add_class(format!("button--{:?}--disabled", self.style).to_lowercase().as_str())
        } else {
            self.clone()
        }
    }
    pub fn action(&mut self, url: &str) -> Self {
        self.set_attr("href", url)
    }

    fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }
}

impl Renderable for Button {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "button",
            include_str!("../themes/components/button.scss"),
        );
        script_registery.register_script(
            "button",
            include_str!("../js/button.js"),
        );
        let mut button = self.clone()
            .add_class("button")
            .add_class(format!("button--{:?}", self.style).to_lowercase().as_str())
            .set_attr("role", "button")
            .tag("a");
        let text = Text::new(self.label.as_str(), TextStyle::Button).render(style_registery, script_registery);
        button.view.children.push(text);
        button.view
    }
}