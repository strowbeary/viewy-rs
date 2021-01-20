use crate::{StyleRegistery, Renderable};
use std::sync::Mutex;
use crate::{component_style, component_script};
use crate::template_compilation_tools::ScriptRegistry;
use crate::view::{View, DefaultModifiers};

#[derive(Clone, Debug)]
pub enum ButtonStyle {
    Link,
    Flat,
    Outlined,
    Filled,
}

#[derive(Clone)]
pub struct Button {
    view: View,
    pub label: String,
    pub style: ButtonStyle,
}

impl DefaultModifiers<Button> for Button {}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            view: View::default(),
            label: label.to_string(),
            style: ButtonStyle::Outlined,
        }
            .add_class("button")
            .add_class("text--button")
            .tag("a")
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
}

impl Renderable for Button {
    fn register_css(&self, style_registery: &mut StyleRegistery) {
        style_registery.register_stylesheet("button", component_style!("button"));
    }

    fn register_js(&self, script_registery: &mut ScriptRegistry) {
        script_registery.register_script("button", component_script!("button"));
    }
}