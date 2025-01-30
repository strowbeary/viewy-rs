use std::borrow::BorrowMut;

use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use html_escape::encode_double_quoted_attribute;
use uuid::Uuid;

use crate::components::icons::Lucide;
use crate::components::*;
use crate::node::{Node, NodeContainer};
use crate::Renderable;
use crate::{scale, sp, DefaultModifiers, Overflow};

fn multi_value_row(
    field_name: &str,
    field_type: &FieldType,
    value: &str,
    form: &Option<String>,
) -> HStack {
    HStack::new(Alignment::Center)
        .flex_grow(1)
        .add_class("field--multi-value__value-list__value")
        .gap(vec![scale(2)])
        .append_child({
            let mut input = Field::new(field_name, field_type.clone())
                .flex_grow(1)
                .value(value);
            if let Some(form) = form {
                input.set_attr("form", form);
            }
            input
        })
        .append_child({ Button::icon_only(Lucide::Trash2, ButtonStyle::Flat).destructive() })
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Password,
    Email,
    Number,
    Tel,
    Date,
    DateTimeLocal,
    /// Each duration will be bound to a Tag component. On click, the duration is added to the start date and injected in end_date field.
    Duration(Vec<Duration>),
    Month,
    Search,
    Time,
    Url,
    Week,
    Hidden,
    TextArea,
    RichTextArea,
}

#[derive(Debug, Clone)]
pub struct Field {
    node: Node,
    pub label: Option<String>,
    pub value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub helper_text: Option<String>,
    pub placeholder: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub field_type: FieldType,
    pub name: String,
    pub auto_sizing: bool,
    pub datalist: bool,
    pub required: bool,
    pub read_only: bool,
    pub disabled: bool,
    pub multiple: Option<Vec<String>>,
    pub form: Option<String>,
}

impl NodeContainer for Field {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Field> for Field {}

impl Field {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        Field {
            node: Default::default(),
            label: None,
            value: None,
            min: None,
            max: None,
            step: None,
            helper_text: None,
            placeholder: None,
            leading_icon: None,
            trailing_icon: None,
            field_type,
            name: name.to_string(),
            auto_sizing: false,
            datalist: false,
            required: false,
            read_only: false,
            disabled: false,
            multiple: None,
            form: None,
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn value(&mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self.clone()
    }

    pub fn min(&mut self, value: &str) -> Self {
        self.min = Some(value.to_string());
        self.clone()
    }

    pub fn max(&mut self, value: &str) -> Self {
        self.max = Some(value.to_string());
        self.clone()
    }

    pub fn step(&mut self, value: &str) -> Self {
        self.step = Some(value.to_string());
        self.clone()
    }

    pub fn read_only(&mut self, is_read_only: bool) -> Self {
        self.read_only = is_read_only;
        self.clone()
    }

    /// Activable only on non textarea fields
    pub fn multiple_value(&mut self, values: Vec<String>) -> Self {
        self.multiple = Some(values);
        self.clone()
    }

    pub fn attach_to_form(&mut self, form_name: &str) -> Self {
        self.form = Some(form_name.to_string());
        self.clone()
    }

    pub fn helper_text(&mut self, helper_text: &str) -> Self {
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }

    pub fn error_message(&mut self, helper_text: &str) -> Self {
        self.add_class("field--error");
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }

    pub fn placeholder(&mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self.clone()
    }

    pub fn trailing_icon(&mut self, name: &str) -> Self {
        self.trailing_icon = Some(name.to_string());
        self.clone()
    }

    pub fn leading_icon(&mut self, name: &str) -> Self {
        self.leading_icon = Some(name.to_string());
        self.clone()
    }

    pub fn async_datalist(&mut self, url: &str) -> Self {
        self.datalist = true;
        self.set_attr("data-async-datalist", url)
    }

    pub fn required(&mut self, is_required: bool) -> Self {
        self.required = is_required;
        self.clone()
    }

    pub fn submit_on_keypress(&mut self) -> Self {
        self.set_attr("data-submit-on-keypress", "true")
    }
    pub fn disabled(&mut self) -> Self {
        self.disabled = true;
        self.clone()
    }
}

impl Renderable for Field {
    fn render(&self) -> Node {
        let mut field = self.clone().add_class("field");
        match &self.field_type {
            FieldType::Hidden => {
                let mut input = field
                    .add_class("field__input")
                    .tag("input")
                    .set_attr("id", self.name.as_str())
                    .set_attr("name", self.name.as_str())
                    .set_attr("type", "hidden");
                if let Some(form) = &self.form {
                    input.set_attr("form", form);
                }

                if let Some(value) = field.value {
                    input.set_attr("value", &value);
                }
                input.node
            }
            FieldType::RichTextArea => {
                field.add_class("field__rich-text-area");
                let mut input = View::new()
                    .tag("input")
                    .add_class("field__input")
                    .set_attr("type", "text")
                    .display("none")
                    .set_attr("id", self.name.as_str())
                    .set_attr("name", self.name.as_str())
                    .set_attr(
                        "value",
                        &encode_double_quoted_attribute(&field.value.unwrap_or_default()),
                    );

                if self.required {
                    input.set_attr("required", "required");
                    field.node.children.push({
                        Text::new("Requis", TextStyle::Caption)
                            .color("var(--color-text-secondary)")
                            .grid_area("required")
                            .render()
                    });
                }
                if self.read_only {
                    input.set_attr("readonly", "readonly");
                }

                let id = Uuid::new_v4().to_string();
                let editor_id = &format!("editor-{}", id);
                field.node.children.push(
                    {
                        Card::new(CardStyle::Outlined)
                            .add_class("field__card")
                            .grid_area("input")
                            .append_child(input)
                            .append_child({
                                let mut editor = View::new()
                                    .add_class("field__editor")
                                    .padding(vec![scale(3)])
                                    .set_attr("id", editor_id);
                                editor.node.text = self.value.clone();
                                editor
                            })
                    }
                    .render(),
                );

                if let Some(label) = field.label {
                    let text = Text::new(label.as_str(), TextStyle::Label)
                        .add_class("field__label")
                        .set_attr("for", self.name.as_str())
                        .tag("label");
                    field.node.children.push(text.render());
                }

                if let Some(helper_text) = field.helper_text {
                    let text = ComplexText::new(helper_text.as_str(), TextStyle::Caption)
                        .add_class("field__helper-text");
                    field.node.children.push(text.render());
                }

                field.node
            }
            FieldType::Duration(values) => {
                let default_start_date = field
                    .value
                    .clone()
                    .and_then(|value| {
                        NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%R")
                            .map(|naive_datetime| Utc.from_utc_datetime(&naive_datetime))
                            .ok()
                    })
                    .unwrap_or(Utc::now());
                field.add_class("field--duration");

                field.node.children.push(
                    {
                        VStack::new(Alignment::Stretch)
                            .gap(vec![scale(3)])
                            .append_child({
                                Field::new(
                                    &format!("{}_start_datetime", self.name),
                                    FieldType::DateTimeLocal,
                                )
                                .required(self.required)
                                .label("Date de début")
                                .value(&default_start_date.format("%FT%R").to_string())
                                .set_attr("id", &format!("{}_start_datetime", self.name))
                                .set_attr("name", &format!("{}_start_datetime", self.name))
                            })
                            .append_child({
                                let mut tags = HStack::new(Alignment::Center).gap(vec![scale(2)]);

                                let mut f = timeago::Formatter::with_language(
                                    timeago::languages::french::French,
                                );
                                f.ago("");
                                for duration in values {
                                    tags.append_child({
                                        Tag::new(&format!(
                                            "+ {}",
                                            f.convert(std::time::Duration::from_millis(
                                                duration.num_milliseconds() as u64
                                            ))
                                        ))
                                        .add_class("clickable")
                                        .set_attr(
                                            "data-duration",
                                            &duration.num_minutes().to_string(),
                                        )
                                    });
                                }
                                tags
                            })
                            .append_child({
                                Field::new(
                                    &format!("{}_end_datetime", self.name),
                                    FieldType::DateTimeLocal,
                                )
                                .required(self.required)
                                .label("Date de fin")
                                .set_attr("id", &format!("{}_end_datetime", self.name))
                                .set_attr("name", &format!("{}_end_datetime", self.name))
                            })
                    }
                    .render(),
                );

                field.node
            }
            _ => {
                if let Some(values) = &self.multiple {
                    field.add_class("field--multi-value");

                    field.node.children.push(
                        {
                            Card::new(CardStyle::Filled)
                                .add_class("field--multi-value__multi-value-container")
                                .padding(vec![scale(3)])
                                .append_child({
                                    let mut field_input = VStack::new(Alignment::Stretch)
                                        .gap(vec![scale(3)])
                                        .append_child({
                                            let mut value_list = VStack::new(Alignment::Stretch)
                                                .add_class("field--multi-value__value-list")
                                                .gap(vec![scale(3)])
                                                .append_child({
                                                    View::new()
                                                        .tag("template")
                                                        .id("value-template")
                                                        .append_child({
                                                            multi_value_row(
                                                                &self.name,
                                                                &self.field_type,
                                                                "",
                                                                &self.form,
                                                            )
                                                        })
                                                });
                                            for value in values.iter() {
                                                value_list.append_child(multi_value_row(
                                                    &self.name,
                                                    &self.field_type,
                                                    value,
                                                    &self.form,
                                                ));
                                            }

                                            value_list
                                        });

                                    if !self.disabled && !self.read_only {
                                        field_input.append_child({
                                            Button::icon_only(Lucide::Plus, ButtonStyle::Outlined)
                                                .add_class("field--multi-value__add-value-button")
                                        });
                                    }
                                    field_input
                                })
                        }
                        .render(),
                    );

                    if let Some(label) = &field.label {
                        let text = Text::new(label, TextStyle::Label)
                            .add_class("field__label")
                            .set_attr("for", self.name.as_str())
                            .tag("label");
                        field.add_class("not-empty");
                        field.node.children.push(text.render());
                    }

                    field.node
                } else {
                    let mut input = View::new()
                        .tag(&match &self.field_type {
                            FieldType::TextArea => "textarea",
                            _ => "input",
                        })
                        .add_class("field__input")
                        .set_attr("id", self.name.as_str())
                        .set_attr("name", self.name.as_str());

                    if self.disabled {
                        input.set_attr("disabled", "disabled");
                    }
                    if self.read_only {
                        input.set_attr("readonly", "readonly");
                    }

                    if let Some(form) = &self.form {
                        input.set_attr("form", form);
                    }

                    if self.required {
                        input.set_attr("required", "required");
                        field.add_class("not-empty");
                        field.node.children.push({
                            Text::new("Requis", TextStyle::Caption)
                                .color("var(--color-text-secondary)")
                                .grid_area("required")
                                .render()
                        });
                    }
                    if let Some(placeholder) = &self.placeholder {
                        input.set_attr("placeholder", placeholder);
                    }

                    match &self.field_type {
                        FieldType::TextArea => {}
                        _ => {
                            input.set_attr(
                                "type",
                                &match &field.field_type {
                                    FieldType::DateTimeLocal => "datetime-local".to_string(),
                                    field_type => format!("{:?}", field_type).to_lowercase(),
                                },
                            );
                        }
                    }
                    if self.datalist && !matches!(field.field_type, FieldType::TextArea) {
                        let id = Uuid::new_v4().to_string();
                        input.set_attr("list", id.as_str());
                    }

                    if let Some(value) = &field.value {
                        match &self.field_type {
                            FieldType::TextArea => {
                                input.node.text = Some(value.clone());
                            }
                            _ => {
                                input.set_attr("value", value);
                            }
                        }
                    }
                    if let Some(value) = &field.min {
                        input.set_attr("min", value);
                    }
                    if let Some(value) = &field.max {
                        input.set_attr("max", value);
                    }
                    if let Some(value) = &field.step {
                        input.set_attr("step", value);
                    }

                    field.node.children.push(input.render());

                    if let Some(placeholder) = &field.placeholder {
                        input.set_attr("placeholder", placeholder);
                    }

                    if let Some(label) = &field.label {
                        let text = Text::new(label, TextStyle::Label)
                            .add_class("field__label")
                            .set_attr("for", self.name.as_str())
                            .tag("label");
                        field.add_class("not-empty");
                        field.node.children.push(text.render());
                    }

                    if let Some(helper_text) = &field.helper_text {
                        let text = ComplexText::new(helper_text, TextStyle::Caption)
                            .add_class("field__helper-text");
                        field.add_class("not-empty");
                        field.node.children.push(text.render());
                    }

                    field.node
                }
            }
        }
    }
}
