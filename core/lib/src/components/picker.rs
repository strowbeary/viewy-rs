use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::{DefaultModifiers, scale};
use crate::components::*;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

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
    children: Vec<Box<dyn Renderable>>,
    style: PickerStyle,
    label: Option<String>,
    name: String,
    value: String,
    options: Vec<PickerOption>,
    is_disabled: bool,
}

impl Picker {
    pub fn new(name: &str, value: &str, picker_style: PickerStyle) -> Self {
        Picker {
            node: Default::default(),
            style: picker_style,
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            children: vec![],
            options: vec![],
            is_disabled: false,
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn submit_on_change(&mut self, submit_on_change: bool) -> Self {
        if submit_on_change {
            self.set_attr("data-auto-submit", "data-auto-submit")
        } else {
            self.unset_attr("data-auto-submit")
        }
    }

    pub fn multiple(&mut self) -> Self {
        self.clone()
    }

    /// Make the button submit specified form
    /// ```rust
    ///View::new()
    ///    .append_child({
    ///        Form::new("formName", "/")
    ///    })
    ///    .append_child({
    ///        Button::new("Submit", ButtonStyle::Filled)
    ///            .attach_to_form("formName")
    ///        })
    /// ```
    pub fn attach_to_form(&mut self, form_name: &str) -> Self {
        self
            .set_attr("form", form_name)
    }

    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self.clone()
    }

    pub fn append_child(&mut self, child: PickerOption) -> Self
    {
        self.options.push(child);
        self.clone()
    }
}


impl NodeContainer for Picker {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Picker> for Picker {}

impl ChildContainer for Picker {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}


impl Renderable for Picker {
    fn render(&self) -> Node {
        let mut picker = self.clone()
            .add_class("picker")
            .add_class(format!("picker--{:?}", self.style).to_lowercase().as_str());

        if let Some(label) = &picker.label {
            let text = Text::new(label.as_str(), TextStyle::Label);
            picker.node.children.push(text.render());
        }


        let picker_id = Uuid::new_v4().to_string();
        match self.style {
            PickerStyle::Segmented => {
                if picker.is_disabled {
                    picker.add_class("picker--segmented--disabled");
                }
                picker.node.children.push({
                    let mut option_list = HStack::new(Alignment::Stretch)
                        .add_class("picker--segmented__option-list");
                    for option in picker.options {
                        let radio_id = format!("picker-segmented-{}-{}", picker_id, option.label);
                        option_list
                            .append_child({
                                let mut radio = View::new()
                                    .tag("input")
                                    .set_attr("type", "radio")
                                    .set_attr("name", self.name.as_str())
                                    .set_attr("value", option.value.as_str())
                                    .set_attr("id", radio_id.as_str())
                                    .add_class("picker--segmented__option-list__radio");
                                if picker.value.eq(option.value.as_str()) {
                                    radio.set_attr("checked", "checked");
                                }
                                radio
                            });
                        option_list.append_child({
                            let mut item = HStack::new(Alignment::Center)
                                .add_class("picker--segmented__option-list__option")
                                .tag("label")
                                .set_attr("for", radio_id.as_str());

                            if let Some(icon) = option.icon {
                                item.append_child({
                                    Icon::new(icon.as_str())
                                        .size(16)
                                        .margin_right(scale(2))
                                });
                            }
                            item.append_child({
                                Text::new(option.label.as_str(), TextStyle::Button)
                            });
                            if picker.value.eq(option.value.as_str()) {
                                item.add_class("selected");
                            }
                            item
                        });
                    }
                    option_list.render()
                })
            }
            PickerStyle::Dropdown => {
                let dropdown_id = format!("picker-dropdown-{}", self.name.as_str());

                picker.node.children.push({
                    View::new()
                        .add_class("picker--dropdown__input")
                        .append_child({
                            TextField::new(self.name.as_str(), FieldType::Hidden)
                                .value(&picker.value)
                                .add_class("picker--dropdown__input__field")
                        })
                        .append_child({
                            Text::new("", TextStyle::Body)
                                .add_class("picker--dropdown__input__value-display")
                                .flex_grow(1)
                        })
                        .popover({
                            Popover::new()
                                .placement(Placement::BottomStart)
                                .add_class("picker--dropdown__dropdown")
                                .hide_arrow()
                                .append_child({
                                    VStack::new(Alignment::Stretch)
                                        .gap(vec![scale(3)])
                                        .append_child({
                                            TextField::new("", FieldType::Search)
                                                .add_class("picker--dropdown__dropdown__search-bar")
                                                .placeholder("Chercher parmis les options...")
                                        })
                                        .append_child({
                                            let mut option_list = VStack::new(Alignment::Stretch)
                                                .add_class("picker--dropdown__dropdown__option-list");
                                            for option in picker.options {
                                                let radio_id = format!("{}-{}", dropdown_id, option.label);

                                                option_list.append_child({
                                                    HStack::new(Alignment::Center)
                                                        .gap(vec![scale(3)])
                                                        .add_class("text--label")
                                                        .add_class("picker--dropdown__dropdown__option-list__option")
                                                        .tag("label")
                                                        .set_attr("for", radio_id.as_str())
                                                        .append_child({
                                                            let mut radio = View::new()
                                                                .tag("input")
                                                                .set_attr("type", "radio")
                                                                .set_attr("name", self.name.as_str())
                                                                .set_attr("value", option.value.as_str())
                                                                .set_attr("id", radio_id.as_str());
                                                            if picker.value.eq(&option.value) {
                                                                radio.set_attr("checked", "checked");
                                                            }
                                                            radio
                                                        })
                                                        .append_child({
                                                            Text::new(&option.label, TextStyle::Label)
                                                                .flex_grow(1)
                                                                .no_wrap(true)
                                                        })
                                                        .append_child({
                                                            Icon::new("check")
                                                                .size(scale(4))
                                                        })
                                                });
                                            }
                                            option_list
                                        })
                                })
                        })
                        .render()
                });
            }
            PickerStyle::RadioGroup => {
                picker.node.children.push({
                    let mut option_list = VStack::new(Alignment::Stretch)
                        .add_class("picker__option-list")
                        .gap(vec![scale(3)]);
                    for option in picker.options {
                        option_list.append_child({
                            let mut radio_row = HStack::new(Alignment::Center)
                                .gap(vec![scale(2)]);
                            let radio_id = format!("picker-radio-{}-{}", self.name.as_str(), option.label);
                            let mut radio_button = View::new()
                                .tag("input")
                                .set_attr("type", "radio")
                                .set_attr("name", self.name.as_str())
                                .set_attr("id", radio_id.as_str())
                                .set_attr("value", option.value.as_str());
                            if picker.is_disabled {
                                radio_button.set_attr("disabled", "disabled");
                            }
                            if picker.value.eq(option.value.as_str()) {
                                radio_button.set_attr("checked", "checked");
                            }
                            radio_row.append_child(
                                radio_button
                            );
                            radio_row.append_child(
                                Text::new(option.label.as_str(), TextStyle::Body)
                                    .set_attr("for", radio_id.as_str())
                                    .tag("label")
                            );

                            radio_row
                        });
                    }
                    option_list.render()
                })
            }
        }

        picker.node
    }
}