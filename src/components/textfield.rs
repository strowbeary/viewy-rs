use crate::{StyleRegistry, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{Node, DefaultModifiers, NodeContainer};
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle, View};

#[derive(Debug, Clone)]
pub struct TextField {
    pub node: Node,
    pub label: Option<String>,
    pub helper_text: Option<String>,
    pub placeholder: Option<String>,
    pub field_type: String,
    pub name: String,
    pub auto_sizing: bool,
}

impl NodeContainer for TextField {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<TextField> for TextField {}

impl TextField {
    pub fn new(name: &str, field_type: &str) -> Self {
        TextField {
            node: Default::default(),
            label: None,
            helper_text: None,
            placeholder: None,
            field_type: field_type.to_string(),
            name: name.to_string(),
            auto_sizing: false
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn helper_text(&mut self, helper_text: &str) -> Self {
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }
}

impl Renderable for TextField {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "textfield",
            include_str!("../themes/components/textfield.scss"),
        );
        let mut field = self.clone()
            .add_class("textfield");

        let mut input = View::new()
            .tag("input")
            .add_class("textfield__input")
            .set_attr("type", field.field_type.as_str());

        if let Some(placeholder) = field.placeholder {
            input.set_attr("placeholder", placeholder.as_str());
        }

        if let Some(label) = field.label {
            let text = Text::new(label.as_str(), TextStyle::Label);
            field.node.children.push(text.render(style_registery, script_registery));
        }
        field.node.children.push(input.render(style_registery, script_registery));
        if let Some(helper_text) = field.helper_text {
            let text = Text::new(helper_text.as_str(), TextStyle::Caption)
                .color("var(--color-text-secondary)");
            field.node.children.push(text.render(style_registery, script_registery));
        }
        field.node
    }
}