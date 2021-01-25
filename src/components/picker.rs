use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistry};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle, HStack, Alignment};

#[derive(Debug, Clone)]
pub enum PickerStyle {
    Segmented,
    Dropdown,
    RadioGroup
}

#[derive(Debug, Clone)]
pub struct Picker {
    pub node: Node,
    pub style: PickerStyle,
    pub label: Option<String>,
    pub name: String,
    pub value: String,
    pub options: Vec<(String, String)>

}
impl NodeContainer for Picker {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Picker> for Picker {}

impl Picker {
    pub fn new(name: &str, value: &str, picker_style: PickerStyle) -> Self {
        Picker {
            node: Default::default(),
            style: picker_style,
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            options: vec![]
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }


    pub fn options(&mut self, options: Vec<(&str, &str)>) -> Self {
        self.options = options.iter()
            .map(|&(label, value)| {
                (label.to_string(), value.to_string())
            })
            .collect();
        self.clone()
    }
}

impl Renderable for Picker {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "picker",
            include_str!("../themes/components/picker.scss"),
        );
        let mut picker = self.clone()
            .add_class("picker")
            .add_class(format!("picker--{:?}", self.style).to_lowercase().as_str());

        if let Some(label) = picker.label {
            let text = Text::new(label.as_str(), TextStyle::Label);
            picker.node.children.push(text.render(style_registery, script_registery));
        }
        match self.style {
            PickerStyle::Segmented => {
                picker.node.children.push({
                    let mut option_list = HStack::new(Alignment::Stretch)
                        .add_class("picker__option-list");
                    for option in picker.options {
                        option_list.add_view_child({
                            let mut option_label = Text::new(option.0.as_str(), TextStyle::Button)
                                .add_class("item");
                            if picker.value.eq(option.1.as_str()) {
                                option_label.add_class("selected");
                            }
                            option_label
                        })
                    }
                    option_list.render(style_registery, script_registery)
                })
            }
            PickerStyle::Dropdown => {}
            PickerStyle::RadioGroup => {}
        }

        picker.node
    }
}