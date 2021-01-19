use crate::{View, StyleRegistery};
use std::sync::Mutex;
use crate::{component_style, component_script};
use crate::template_compilation_tools::ScriptRegistry;

#[derive(Clone, Debug)]
pub enum ButtonStyle {
    Link,
    Flat,
    Outlined,
    Filled,
}

#[derive(Clone)]
pub struct Button {
    pub class_list: Vec<String>,
    pub label: String,
    pub style: ButtonStyle,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            class_list: vec!["button".to_string()],
            label: label.to_string(),
            style: ButtonStyle::Outlined,
        }
    }
    pub fn destructive(&mut self) -> Self {
        self.class_list.push(format!("button--{:?}--destructive", self.style).to_lowercase());
        self.clone()
    }
    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        if is_disabled {
            self.class_list.push(format!("button--{:?}--disabled", self.style).to_lowercase());
        }
        self.clone()
    }
}

impl View for Button {
    fn get_html(&self) -> String {
        let classes = self.class_list.join(" ");
        let style_class = format!("button--{:?}", self.style).to_lowercase();
        format!("<button class=\"{} {}\">{}</div>",
                classes,
                style_class,
                self.label,
        )
    }

    fn register_css(&self, style_registery: &mut StyleRegistery) {
        style_registery.register_stylesheet("button", component_style!("button"));
    }

    fn register_js(&self, script_registery: &mut ScriptRegistry) {
        script_registery.register_script("button", component_script!("button"));
    }
}

impl Default for Button {
    fn default() -> Self {
        Button {
            class_list: vec!["button".to_string()],
            label: "Label".to_string(),
            style: ButtonStyle::Outlined,
        }
    }
}