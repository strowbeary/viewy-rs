use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, scale};
use crate::renderer::{ToHtml, Renderable, StyleRegistry, ScriptRegistry};
use crate::components::{Text, TextStyle, HStack, Alignment, Icon, VStack, View};

#[derive(Debug, Clone)]
pub struct PickerOption {
    pub icon: Option<String>,
    pub label: String,
    pub value: String,
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

impl ToHtml for Picker {}

impl Picker {
    pub fn new(name: &str, value: &str, picker_style: PickerStyle) -> Self {
        Picker {
            node: Default::default(),
            style: picker_style,
            label: None,
            name: name.to_string(),
            value: value.to_string(),
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
                                item.add_view_child({
                                    Icon::new(icon.as_str())
                                        .size(16)
                                        .margin_right(scale(2))
                                });
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
            PickerStyle::Dropdown => {}
            PickerStyle::RadioGroup => {
                picker.node.children.push({
                    let mut option_list = VStack::new(Alignment::Stretch)
                        .add_class("picker__option-list")
                        .gap(vec![scale(3)]);
                    for option in picker.options {
                        option_list.add_view_child({
                            let mut radio_row = HStack::new(Alignment::Center)
                                .gap(vec![scale(2)]);
                            let radio_id = format!("picker-radio-{}-{}", self.name.as_str(), option.label);
                            let mut radio_button = View::new()
                                .tag("input")
                                .set_attr("type", "radio")
                                .set_attr("name", self.name.as_str())
                                .set_attr("id", radio_id.as_str());
                            if picker.value.eq(option.value.as_str()) {
                                radio_button.set_attr("checked", "checked");
                            }
                            radio_row.add_view_child(
                                radio_button
                            );
                            radio_row.add_view_child(
                                Text::new(option.label.as_str(), TextStyle::Body)
                                    .set_attr("for", radio_id.as_str())
                                    .tag("label")
                            );

                            radio_row
                        })
                    }
                    option_list.render(style_registery, script_registery)
                })
            }
        }

        picker.node
    }
}