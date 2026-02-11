use crate::Widget;
use crate::core::node::{Node, NodeType};
use crate::modifiers::{Attributable, Classable};
use crate::prelude::{Icon, IconPack, Lucide, View};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SelectOption {
    pub icon: Option<Box<dyn IconPack>>,
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl SelectOption {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            icon: None,
            label: label.to_string(),
            value: value.to_string(),
            selected: false,
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
        self.selected = true;
        self
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SelectGroup {
    pub label: String,
    pub options: Vec<SelectOption>,
}

impl SelectGroup {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            options: vec![],
        }
    }

    pub fn append_option(&mut self, option: SelectOption) -> &mut Self {
        self.options.push(option);
        self
    }
}

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.css")]
pub struct Select {
    node: Node,
    label: Option<String>,
    placeholder: String,
    name: String,
    value: String,
    options: Vec<SelectOption>,
    groups: Vec<SelectGroup>,
    is_disabled: bool,
    auto_submit: bool,
    required: bool,
    form: Option<String>,
    searchable: bool,
}

impl Select {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            node: Default::default(),
            label: None,
            placeholder: "Select an option".to_string(),
            name: name.to_string(),
            value: value.to_string(),
            options: vec![],
            groups: vec![],
            is_disabled: false,
            auto_submit: false,
            required: false,
            form: None,
            searchable: true,
        }
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder = placeholder.to_string();
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

    pub fn searchable(&mut self, searchable: bool) -> &mut Self {
        self.searchable = searchable;
        self
    }

    pub fn disable_search_bar(&mut self) -> &mut Self {
        self.searchable(false)
    }

    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
        self.form = Some(form_name.to_string());
        self
    }

    pub fn disabled(&mut self, is_disabled: bool) -> &mut Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn append_option(&mut self, option: SelectOption) -> &mut Self {
        self.options.push(option);
        self
    }

    pub fn append_group(&mut self, group: SelectGroup) -> &mut Self {
        self.groups.push(group);
        self
    }

    fn resolve_selected(&self) -> Option<&SelectOption> {
        let from_value = self
            .options
            .iter()
            .find(|option| self.value == option.value)
            .or_else(|| {
                self.groups
                    .iter()
                    .flat_map(|group| group.options.iter())
                    .find(|option| self.value == option.value)
            });

        from_value.or_else(|| {
            self.options
                .iter()
                .find(|option| option.selected)
                .or_else(|| {
                    self.groups
                        .iter()
                        .flat_map(|group| group.options.iter())
                        .find(|option| option.selected)
                })
        })
    }

    fn build_option(
        option: &SelectOption,
        option_id: &str,
        is_selected: bool,
        option_index: usize,
    ) -> Node {
        let mut option_node = View::new();
        option_node
            .add_class("select__option")
            .set_attr("type", "button")
            .set_attr("role", "option")
            .set_attr("tabindex", "-1")
            .set_attr("data-value", &option.value)
            .set_attr("data-label", &option.label)
            .set_attr("data-index", &option_index.to_string())
            .set_attr("id", option_id);
        option_node.node.node_type = NodeType::Normal("button");

        if is_selected {
            option_node
                .add_class("is-selected")
                .set_attr("aria-selected", "true");
        } else {
            option_node.set_attr("aria-selected", "false");
        }

        if let Some(option_icon) = option.icon.clone() {
            let mut icon_container = View::new();
            icon_container
                .add_class("select__option-icon")
                .set_attr("aria-hidden", "true");
            icon_container.node.node_type = NodeType::Normal("span");

            let mut icon = Icon::new(option_icon);
            icon.size(16);
            icon_container.node.children.push(icon.into());

            option_node.node.children.push(icon_container.into());
        } else {
            let mut placeholder = View::new();
            placeholder
                .add_class("select__option-icon")
                .set_attr("aria-hidden", "true");
            placeholder.node.node_type = NodeType::Normal("span");
            option_node.node.children.push(placeholder.into());
        }

        let mut option_label = View::new();
        option_label.add_class("select__option-label");
        option_label.node.node_type = NodeType::Normal("span");
        option_label.text = Some(option.label.clone());
        option_node.node.children.push(option_label.into());

        let mut check_icon = Icon::new(Lucide::Check);
        check_icon.size(16).add_class("select__check");
        option_node.node.children.push(check_icon.into());

        option_node.into()
    }

    fn render(&mut self) {
        self.add_class("select");
        self.set_attr("data-v-select", "true");
        if self.is_disabled {
            self.add_class("select--disabled");
        }

        let select_id = format!("select-{}", Uuid::new_v4());
        let label_id = format!("{select_id}-label");
        let trigger_id = format!("{select_id}-trigger");
        let value_id = format!("{select_id}-value");
        let panel_id = format!("{select_id}-panel");
        let search_id = format!("{select_id}-search");
        let listbox_id = format!("{select_id}-listbox");

        if let Some(label) = &self.label {
            let mut label_node = View::new();
            label_node
                .add_class("select__label")
                .set_attr("id", &label_id)
                .set_attr("for", &trigger_id);
            label_node.node.node_type = NodeType::Normal("label");
            label_node.text = Some(label.clone());
            self.node.children.push(label_node.into());
        }

        let resolved_selected = self
            .resolve_selected()
            .map(|option| (option.value.clone(), option.label.clone()));

        let selected_value = resolved_selected
            .as_ref()
            .map(|(value, _)| value.clone())
            .unwrap_or_else(|| self.value.clone());
        let selected_label = resolved_selected
            .as_ref()
            .map(|(_, label)| label.clone())
            .unwrap_or_else(|| self.placeholder.clone());
        let selected_value_key = if selected_value.is_empty() {
            None
        } else {
            Some(selected_value.clone())
        };

        let mut hidden_field = View::new();
        hidden_field
            .add_class("select__field")
            .set_attr("type", "hidden")
            .set_attr("name", &self.name)
            .set_attr("value", &selected_value);
        hidden_field.node.node_type = NodeType::SelfClosing("input");

        if let Some(form_id) = &self.form {
            hidden_field.set_attr("form", form_id);
        }
        if self.auto_submit {
            hidden_field.set_attr("data-auto-submit", "true");
        }
        if self.required {
            hidden_field.set_attr("required", "required");
        }

        self.node.children.push(hidden_field.into());

        let mut trigger = View::new();
        trigger
            .add_class("select__trigger")
            .set_attr("type", "button")
            .set_attr("id", &trigger_id)
            .set_attr("role", "combobox")
            .set_attr("aria-haspopup", "listbox")
            .set_attr("aria-expanded", "false")
            .set_attr("aria-controls", &listbox_id)
            .set_attr("aria-live", "polite")
            .set_attr("data-v-select-trigger", "true");
        trigger.node.node_type = NodeType::Normal("button");

        if self.label.is_some() {
            trigger.set_attr("aria-labelledby", &format!("{label_id} {value_id}"));
        }
        if self.required {
            trigger.set_attr("aria-required", "true");
        }
        if self.is_disabled {
            trigger
                .set_attr("disabled", "disabled")
                .set_attr("aria-disabled", "true");
        }

        let mut value_display = View::new();
        value_display
            .add_class("select__value")
            .set_attr("id", &value_id);
        value_display.node.node_type = NodeType::Normal("span");
        value_display.text = Some(selected_label);
        trigger.node.children.push(value_display.into());

        let mut chevron_icon = Icon::new(Lucide::ChevronDown);
        chevron_icon.size(16).add_class("select__chevron");
        trigger.node.children.push(chevron_icon.into());

        self.node.children.push(trigger.into());

        let mut panel = View::new();
        panel
            .add_class("select__panel")
            .set_attr("id", &panel_id)
            .set_attr("data-v-select-panel", "true")
            .set_attr("hidden", "hidden");
        panel.node.node_type = NodeType::Normal("div");

        if self.searchable {
            let mut search = View::new();
            search
                .add_class("select__search")
                .set_attr("id", &search_id)
                .set_attr("type", "search")
                .set_attr("placeholder", "Search")
                .set_attr("autocomplete", "off")
                .set_attr("spellcheck", "false");
            search.node.node_type = NodeType::SelfClosing("input");
            panel.node.children.push(search.into());
        }

        let mut listbox = View::new();
        listbox
            .add_class("select__listbox")
            .set_attr("id", &listbox_id)
            .set_attr("role", "listbox")
            .set_attr("tabindex", "-1");
        listbox.node.node_type = NodeType::Normal("div");

        let mut option_index = 0usize;
        let mut selected_option_id: Option<String> = None;

        for option in &self.options {
            let option_id = format!("{select_id}-option-{option_index}");
            let is_selected = selected_value_key
                .as_ref()
                .map(|selected| selected == &option.value)
                .unwrap_or(false);
            if is_selected {
                selected_option_id = Some(option_id.clone());
            }

            listbox.node.children.push(Self::build_option(
                option,
                &option_id,
                is_selected,
                option_index,
            ));
            option_index += 1;
        }

        for group in &self.groups {
            let mut group_node = View::new();
            group_node
                .add_class("select__group")
                .set_attr("role", "group")
                .set_attr("aria-label", &group.label);
            group_node.node.node_type = NodeType::Normal("div");

            let mut group_label = View::new();
            group_label.add_class("select__group-label");
            group_label.node.node_type = NodeType::Normal("div");
            group_label.text = Some(group.label.clone());
            group_node.node.children.push(group_label.into());

            for option in &group.options {
                let option_id = format!("{select_id}-option-{option_index}");
                let is_selected = selected_value_key
                    .as_ref()
                    .map(|selected| selected == &option.value)
                    .unwrap_or(false);
                if is_selected {
                    selected_option_id = Some(option_id.clone());
                }

                group_node.node.children.push(Self::build_option(
                    option,
                    &option_id,
                    is_selected,
                    option_index,
                ));
                option_index += 1;
            }

            listbox.node.children.push(group_node.into());
        }

        if let Some(selected_id) = selected_option_id {
            listbox.set_attr("aria-activedescendant", &selected_id);
        }

        panel.node.children.push(listbox.into());
        self.node.children.push(panel.into());
    }
}
