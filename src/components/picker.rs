use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistry};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle, HStack, Alignment};

#[derive(Debug, Clone)]
pub struct PickerOption {
    icon: Option<String>,
    label: String,
    value: String,
}

impl PickerOption {
    pub fn new(label: &str, value: &str) -> Self {
        PickerOption {
            icon: None,
            label: label.to_string(),
            value: value.to_string(),
        }
    }
    pub fn icon(&mut self, name: &str) -> Self {
        self.icon = Some(name.to_string());
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum PickerStyle {
    Segmented,
    Dropdown,
    RadioGroup,
}

#[derive(Debug, Clone)]
pub struct Picker {
    node: Node,
    style: PickerStyle,
    label: Option<String>,
    name: String,
    value: String,
    options: Vec<PickerOption>,
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
            value: "".to_string(),
            options: vec![],
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn add_view_child(&mut self, child: PickerOption) {
        self.options.push(child);
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
                script_registery.register_script(
                    "picker-segmented",
                    include_str!("../js/picker-segmented.js"),
                );
                picker.node.children.push({
                    let mut option_list = HStack::new(Alignment::Stretch)
                        .add_class("picker__option-list");
                    for option in picker.options {
                        option_list.add_view_child({
                            let mut item = HStack::new(Alignment::Center).add_class("item");
                            if let Some(icon) = option.icon {
                                // item.add_view_child(Text::new(option.label.as_str(), TextStyle::Button));
                            }
                            item.add_view_child(Text::new(option.label.as_str(), TextStyle::Button));
                            if picker.value.eq(option.value.as_str()) {
                                item.add_class("selected");
                            }
                            item
                        })
                    }
                    option_list.render(style_registery, script_registery)
                })
            }
            PickerStyle::Dropdown => {

            }
            PickerStyle::RadioGroup => {}
        }

        picker.node
    }
}