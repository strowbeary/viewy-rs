use std::borrow::BorrowMut;
use std::cmp::PartialEq;
use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use html_escape::encode_double_quoted_attribute;
use uuid::Uuid;

use crate::Renderable;
use crate::components::icons::Lucide;
use crate::components::*;
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, scale};

fn multi_value_row(
    field_name: &str,
    field_type: &FieldType,
    value: &str,
    form: &Option<String>,
) -> HStack {
    let mut stack = HStack::new(Alignment::Center);
    stack.flex_grow(1)
        .add_class("field--multi-value__value-list__value")
        .gap(vec![scale(2)])
        .append_child({
            let mut input = Field::new(field_name, field_type.clone());
            input.flex_grow(1)
                .value(value);
            if let Some(form) = form {
                input.set_attr("form", form);
            }
            input
        })
        .append_child({
            let mut btn = Button::icon_only(Lucide::Trash2, ButtonStyle::Flat);
            btn.destructive();
            btn
        });
    stack
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    MainSearchBar,
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
    pub pattern: Option<String>,
}

impl NodeContainer for Field {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Field {}

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
            pattern: None,
        }
    }

    pub fn label(&mut self, label: &str) -> &mut Self{
        self.label = Some(label.to_string());
        self
    }

    pub fn value(&mut self, value: &str) -> &mut Self{
        self.value = Some(ammonia::clean(value));
        self
    }

    pub fn min(&mut self, value: &str) -> &mut Self{
        self.min = Some(value.to_string());
        self
    }

    pub fn max(&mut self, value: &str) -> &mut Self{
        self.max = Some(value.to_string());
        self
    }

    pub fn step(&mut self, value: &str) -> &mut Self{
        self.step = Some(value.to_string());
        self
    }

    pub fn read_only(&mut self, is_read_only: bool) -> &mut Self{
        self.read_only = is_read_only;
        self
    }

    /// Enable user to input multiple values in a form for the same field.
    /// Will be serialized in the POST request as URL Encoded form data or multipart form data.
    /// For a field defined as following
    /// ```
    /// use viewy::components::{Field, FieldType};
    /// Field::new("name", FieldType::Text)
    ///                .multiple_value(vec![
    ///                     String::from("value1"), 
    ///                     String::from("value2")
    ///                 ])
    /// ```
    /// The form data will look like this
    /// ```x-www-form-urlencoded
    /// name=value1&name=value2
    /// ```
    ///
    /// # Example
    /// To add multiple existing values
    /// ```rust
    /// use viewy::components::{Field, FieldType};
    /// let field = Field::new("name", FieldType::Text)
    ///                .multiple_value(vec![
    ///                     String::from("value1"), 
    ///                     String::from("value2")
    ///                 ]);
    /// ```
    ///
    /// To display an empty multiple value field
    /// ```rust
    /// use viewy::components::{Field, FieldType};
    /// let field = Field::new("name", FieldType::Text)
    ///                 .multiple_value(vec![]);
    /// ```
    pub fn multiple_value(&mut self, values: Vec<String>) -> &mut Self{
        self.multiple = Some(values);
        self
    }

    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self{
        self.form = Some(form_name.to_string());
        self
    }

    pub fn helper_text(&mut self, helper_text: &str) -> &mut Self{
        self.helper_text = Some(helper_text.to_string());
        self
    }

    pub fn error_message(&mut self, helper_text: &str) -> &mut Self{
        self.add_class("field--error");
        self.helper_text = Some(helper_text.to_string());
        self
    }

    pub fn placeholder(&mut self, placeholder: &str) -> &mut Self{
        self.placeholder = Some(placeholder.to_string());
        self
    }

    pub fn trailing_icon(&mut self, name: &str) -> &mut Self{
        self.trailing_icon = Some(name.to_string());
        self
    }

    pub fn leading_icon(&mut self, name: &str) -> &mut Self{
        self.leading_icon = Some(name.to_string());
        self
    }

    pub fn async_datalist(&mut self, url: &str) -> &mut Self{
        self.datalist = true;
        self.set_attr("data-async-datalist", url)
    }

    /// Défini l'attribut standard HTML pattern sur l'<input>
    pub fn pattern(&mut self, pattern: &str) -> &mut Self{
        self.pattern = Some(pattern.to_string());
        self
    }

    pub fn required(&mut self, is_required: bool) -> &mut Self{
        self.required = is_required;
        self
    }

    pub fn submit_on_keypress(&mut self) -> &mut Self{
        self.set_attr("data-submit-on-keypress", "true")
    }
    pub fn disabled(&mut self) -> &mut Self{
        self.disabled = true;
        self
    }
}


impl Renderable for Field {
    fn render(mut self) -> Node {
        self.add_class("field");
        match &self.field_type {
            FieldType::Hidden => {
                self
                    .add_class("field__input")
                    .tag("input")
                    .set_attr("id", self.name.as_str())
                    .set_attr("name", self.name.as_str())
                    .set_attr("type", "hidden");
                if let Some(form) = &self.form {
                    self.set_attr("form", form);
                }

                if let Some(value) = &self.value {
                    self.set_attr("value", value);
                }
                self.node
            }
            FieldType::RichTextArea => {
                self.add_class("field__rich-text-area");
                let mut input = View::new();
                    input.tag("input")
                    .add_class("field__input")
                    .set_attr("type", "text")
                    .display("none")
                    .set_attr("id", self.name.as_str())
                    .set_attr("name", self.name.as_str())
                    .set_attr(
                        "value",
                        &encode_double_quoted_attribute(&self.value.clone().unwrap_or_default()),
                    );

                if self.required {
                    input.set_attr("required", "required");
                    self.node.children.push({
                        let mut text = Text::new("Requis", TextStyle::Caption);
                        text.color("var(--color-text-secondary)")
                            .grid_area("required");
                        text.render()
                    });
                }
                if self.read_only {
                    input.set_attr("readonly", "readonly");
                }

                let id = Uuid::new_v4().to_string();
                let editor_id = &format!("editor-{}", id);
                self.node.children.push(
                    {
                        let mut card = Card::new(CardStyle::Outlined);
                        card.add_class("field__card")
                            .grid_area("input")
                            .append_child(input)
                            .append_child({
                                let mut editor = View::new();
                                editor.add_class("field__editor")
                                    .padding(vec![scale(3)])
                                    .set_attr("id", editor_id);
                                editor.node.text = self.value.clone();
                                editor
                            });
                        card
                    }
                    .render(),
                );

                if let Some(label) = &self.label {
                    let mut text = Text::new(label, TextStyle::Label);
                    text.add_class("field__label")
                        .set_attr("for", &self.name)
                        .tag("label");
                    self.node.children.push(text.render());
                }

                if let Some(helper_text) = &self.helper_text {
                    let mut text = ComplexText::new(helper_text, TextStyle::Caption);
                    text.add_class("field__helper-text");
                    self.node.children.push(text.render());
                }

                self.node
            }
            FieldType::Duration(values) => {
                let default_start_date = self
                    .value
                    .clone()
                    .and_then(|value| {
                        NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%R")
                            .map(|naive_datetime| Utc.from_utc_datetime(&naive_datetime))
                            .ok()
                    })
                    .unwrap_or(Utc::now());
                self.add_class("field--duration");

                self.node.children.push(
                    {
                        let mut stack = VStack::new(Alignment::Stretch);
                        stack.gap(vec![scale(3)])
                            .append_child({
                                let mut field = Field::new(
                                    &format!("{}_start_datetime", self.name),
                                    FieldType::DateTimeLocal,
                                );
                                field.required(self.required)
                                .label("Date de début")
                                .value(&default_start_date.format("%FT%R").to_string())
                                .set_attr("id", &format!("{}_start_datetime", self.name))
                                .set_attr("name", &format!("{}_start_datetime", self.name));
                                field
                            })
                            .append_child({
                                let mut tags = HStack::new(Alignment::Center);
                                tags.gap(vec![scale(2)]);

                                let mut f = timeago::Formatter::with_language(
                                    timeago::languages::french::French,
                                );
                                f.ago("");
                                for duration in values {
                                    tags.append_child({
                                        let mut tag = Tag::new(&format!(
                                            "+ {}",
                                            f.convert(std::time::Duration::from_millis(
                                                duration.num_milliseconds() as u64
                                            ))
                                        ));
                                        tag.add_class("clickable")
                                        .set_attr(
                                            "data-duration",
                                            &duration.num_minutes().to_string(),
                                        );
                                        tag
                                    });
                                }
                                tags
                            })
                            .append_child({
                                let mut field = Field::new(
                                    &format!("{}_end_datetime", self.name),
                                    FieldType::DateTimeLocal,
                                );
                                field.required(self.required)
                                .label("Date de fin")
                                .set_attr("id", &format!("{}_end_datetime", self.name))
                                .set_attr("name", &format!("{}_end_datetime", self.name));
                                field
                            });
                        stack
                    }
                    .render(),
                );

                self.node
            }
            _ => {
                if let Some(values) = &self.multiple {
                    self.add_class("field--multi-value");

                    self.node.children.push(
                        {
                            let mut card = Card::new(CardStyle::Filled);
                            card.add_class("field--multi-value__multi-value-container")
                                .padding(vec![scale(3)])
                                .append_child({
                                    let mut field_input = VStack::new(Alignment::Stretch);
                                    field_input.gap(vec![scale(3)])
                                        .append_child({
                                            let mut value_list = VStack::new(Alignment::Stretch);
                                            value_list.add_class("field--multi-value__value-list")
                                                .gap(vec![scale(3)])
                                                .append_child({
                                                    let mut view = View::new();
                                                    view.tag("template")
                                                        .id("value-template")
                                                        .append_child({
                                                            multi_value_row(
                                                                &self.name,
                                                                &self.field_type,
                                                                "",
                                                                &self.form,
                                                            )
                                                        });
                                                    view
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
                                            let mut btn = Button::icon_only(Lucide::Plus, ButtonStyle::Outlined);
                                            btn.add_class("field--multi-value__add-value-button");
                                            btn
                                        });
                                    }
                                    field_input
                                });
                            card
                        }
                        .render(),
                    );

                    if let Some(label) = &self.label {
                        let text = Text::new(label, TextStyle::Label);
                        text.add_class("field__label")
                            .set_attr("for", self.name.as_str())
                            .tag("label");
                        self.add_class("not-empty");
                        self.node.children.push(text.render());
                    }

                    self.node
                } else {
                    let mut input = View::new();
                    input.tag(&match &self.field_type {
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
                    if let Some(pattern) = &self.pattern {
                        input.set_attr("pattern", pattern);
                    }

                    if self.required {
                        input.set_attr("required", "required");
                        self.add_class("not-empty");
                        self.node.children.push({
                            let mut text = Text::new("Requis", TextStyle::Caption);
                            text.color("var(--color-text-secondary)")
                                .grid_area("required");
                            text.render()
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
                                &match &self.field_type {
                                    FieldType::DateTimeLocal => "datetime-local".to_string(),
                                    FieldType::MainSearchBar => "search".to_string(),
                                    field_type => format!("{:?}", field_type).to_lowercase(),
                                },
                            );
                        }
                    }

                    if self.field_type ==  FieldType::MainSearchBar {
                        input.add_class("field__input--main-search-bar");
                    }
                    if self.datalist && !matches!(self.field_type, FieldType::TextArea) {
                        let id = Uuid::new_v4().to_string();
                        input.set_attr("list", id.as_str());
                    }

                    if let Some(value) = &self.value {
                        match &self.field_type {
                            FieldType::TextArea => {
                                input.node.text = Some(value.clone());
                            }
                            _ => {
                                input.set_attr("value", value);
                            }
                        }
                    }
                    if let Some(value) = &self.min {
                        input.set_attr("min", value);
                    }
                    if let Some(value) = &self.max {
                        input.set_attr("max", value);
                    }
                    if let Some(value) = &self.step {
                        input.set_attr("step", value);
                    }


                    if let Some(placeholder) = &self.placeholder {
                        input.set_attr("placeholder", placeholder);
                    }
                    self.node.children.push(input.render());

                    if let Some(label) = &self.label {
                        let text = Text::new(label, TextStyle::Label)
                            .add_class("field__label")
                            .set_attr("for", self.name.as_str())
                            .tag("label");
                        self.add_class("not-empty");
                        self.node.children.push(text.render());
                    }

                    if let Some(helper_text) = &self.helper_text {
                        let mut text = ComplexText::new(helper_text, TextStyle::Caption);
                        text.add_class("field__helper-text");
                        self.add_class("not-empty");
                        self.node.children.push(text.render());
                    }

                    self.node
                }
            }
        }
    }
}
