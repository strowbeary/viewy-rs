use crate::Widget;
use crate::core::node::{Node, NodeType};
use crate::modifiers::{Attributable, Classable};
use crate::prelude::{Icon, IconPack, View};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PickerOption {
    pub icon: Option<Box<dyn IconPack>>,
    pub label: String,
    pub value: String,
    pub selected: Option<bool>,
}

impl PickerOption {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            icon: None,
            label: label.to_string(),
            value: value.to_string(),
            selected: None,
        }
    }

    pub fn icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self
    }

    pub fn selected(&mut self) -> &mut Self {
        self.selected = Some(true);
        self
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = Some(selected);
        self
    }
}

#[derive(Debug, Clone)]
pub struct PickerGroup {
    pub label: String,
    pub options: Vec<PickerOption>,
}

impl PickerGroup {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            options: vec![],
        }
    }

    pub fn append_option(&mut self, option: PickerOption) -> &mut Self {
        self.options.push(option);
        self
    }
}

#[derive(Debug, Clone)]
pub enum PickerStyle {
    Segmented,
    RadioGroup,
}

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.css")]
pub struct Picker {
    node: Node,
    style: PickerStyle,
    label: Option<String>,
    name: String,
    value: String,
    options: Vec<PickerOption>,
    groups: Vec<PickerGroup>,
    is_disabled: bool,
    auto_submit: bool,
    required: bool,
    multiple: bool,
    segmented_vertical: bool,
    form: Option<String>,
}

impl Picker {
    pub fn new(name: &str, value: &str, picker_style: PickerStyle) -> Self {
        Self {
            node: Default::default(),
            style: picker_style,
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            options: vec![],
            groups: vec![],
            is_disabled: false,
            auto_submit: false,
            required: false,
            form: None,
            multiple: false,
            segmented_vertical: false,
        }
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn required(&mut self) -> &mut Self {
        self.required = true;
        self
    }

    pub fn submit_on_change(&mut self, submit_on_change: bool) -> &mut Self {
        self.auto_submit = submit_on_change;
        self
    }

    pub fn multiple(&mut self) -> &mut Self {
        if matches!(self.style, PickerStyle::Segmented) {
            self.multiple = true;
        }
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        if matches!(self.style, PickerStyle::Segmented) {
            self.segmented_vertical = true;
        }
        self
    }

    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
        self.form = Some(form_name.to_string());
        self
    }

    pub fn disabled(&mut self, is_disabled: bool) -> &mut Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn append_option(&mut self, child: PickerOption) -> &mut Self {
        self.options.push(child);
        self
    }

    pub fn append_group(&mut self, group: PickerGroup) -> &mut Self {
        self.groups.push(group);
        self
    }

    fn is_option_selected(&self, option: &PickerOption, is_multiple: bool) -> bool {
        if is_multiple {
            option.selected.unwrap_or(false)
        } else if !self.value.is_empty() {
            self.value == option.value
        } else {
            option.selected.unwrap_or(false)
        }
    }

    fn build_item(
        &self,
        picker_id: &str,
        option: &PickerOption,
        index: usize,
        is_multiple: bool,
    ) -> Node {
        let input_id = format!("{picker_id}-option-{index}");
        let is_checked = self.is_option_selected(option, is_multiple);

        let mut item = View::new();
        item.add_class("picker__item");
        item.node.node_type = NodeType::Normal("div");

        let mut input = View::new();
        input
            .add_class("picker__item-input")
            .set_attr("id", &input_id)
            .set_attr("name", &self.name)
            .set_attr("value", &option.value)
            .set_attr("type", if is_multiple { "checkbox" } else { "radio" });
        input.node.node_type = NodeType::SelfClosing("input");

        if is_checked {
            input.set_attr("checked", "checked");
        }
        if self.required {
            input.set_attr("required", "required");
        }
        if self.auto_submit {
            input.set_attr("data-auto-submit", "true");
        }
        if self.is_disabled {
            input.set_attr("disabled", "disabled");
        }
        if let Some(form_name) = &self.form {
            input.set_attr("form", form_name);
        }

        item.node.children.push(input.into());

        let mut item_label = View::new();
        item_label
            .add_class("picker__item-label")
            .set_attr("for", &input_id);
        item_label.node.node_type = NodeType::Normal("label");

        if let Some(option_icon) = option.icon.clone() {
            let mut icon_container = View::new();
            icon_container
                .add_class("picker__item-icon")
                .set_attr("aria-hidden", "true");
            icon_container.node.node_type = NodeType::Normal("span");

            let mut icon = Icon::new(option_icon);
            icon.size(16);
            icon_container.node.children.push(icon.into());

            item_label.node.children.push(icon_container.into());
        }

        let mut option_label = View::new();
        option_label.add_class("picker__item-text");
        option_label.node.node_type = NodeType::Normal("span");
        option_label.text = Some(option.label.clone());
        item_label.node.children.push(option_label.into());

        item.node.children.push(item_label.into());
        item.into()
    }

    fn build_group(
        &self,
        picker_id: &str,
        group_label: &str,
        options: &[PickerOption],
        start_index: &mut usize,
        is_multiple: bool,
    ) -> Node {
        let mut group = View::new();
        group.add_class("picker__group");
        group.node.node_type = NodeType::Normal("fieldset");

        if self.is_disabled {
            group.set_attr("disabled", "disabled");
        }

        if !group_label.is_empty() {
            let mut legend = View::new();
            legend.add_class("picker__group-legend");
            legend.node.node_type = NodeType::Normal("legend");
            legend.text = Some(group_label.to_string());
            group.node.children.push(legend.into());
        }

        let mut group_options = View::new();
        group_options.add_class("picker__options");
        group_options.node.node_type = NodeType::Normal("div");

        for option in options {
            group_options.node.children.push(self.build_item(
                picker_id,
                option,
                *start_index,
                is_multiple,
            ));
            *start_index += 1;
        }

        group.node.children.push(group_options.into());
        group.into()
    }

    fn render(&mut self) {
        self.add_class("picker");
        self.set_attr("data-v-picker", "true");

        match self.style {
            PickerStyle::Segmented => {
                self.add_class("picker--segmented");
                if self.segmented_vertical {
                    self.add_class("picker--segmented--vertical");
                }
            }
            PickerStyle::RadioGroup => {
                self.add_class("picker--radiogroup");
            }
        }

        if self.is_disabled {
            self.add_class("picker--disabled");
        }

        let picker_id = format!("picker-{}", Uuid::new_v4());
        let label_id = format!("{picker_id}-label");

        if let Some(label) = &self.label {
            let mut label_node = View::new();
            label_node
                .add_class("picker__label")
                .set_attr("id", &label_id);
            label_node.node.node_type = NodeType::Normal("p");
            label_node.text = Some(label.clone());
            self.node.children.push(label_node.into());
        }

        let is_multiple = matches!(self.style, PickerStyle::Segmented) && self.multiple;

        let mut groups_container = View::new();
        groups_container
            .add_class("picker__groups")
            .set_attr("role", if is_multiple { "group" } else { "radiogroup" });
        groups_container.node.node_type = NodeType::Normal("div");

        if self.label.is_some() {
            groups_container.set_attr("aria-labelledby", &label_id);
        }
        if self.is_disabled {
            groups_container.set_attr("aria-disabled", "true");
        }

        let mut option_index = 0usize;

        if !self.options.is_empty() {
            groups_container.node.children.push(self.build_group(
                &picker_id,
                "",
                &self.options,
                &mut option_index,
                is_multiple,
            ));
        }

        for group in &self.groups {
            groups_container.node.children.push(self.build_group(
                &picker_id,
                &group.label,
                &group.options,
                &mut option_index,
                is_multiple,
            ));
        }

        self.node.children.push(groups_container.into());
    }
}
