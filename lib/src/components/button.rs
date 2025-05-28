use std::borrow::BorrowMut;
use std::ops::DerefMut;

use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::badge::{Badge, BadgeSupport};
use crate::components::icons::IconPack;
use crate::components::{BadgeModifiers, Icon, Text, TextStyle};
use crate::node::Node;

/// Used to set a button's importance level.
#[derive(Debug, Clone)]
pub enum ButtonStyle {
    SmallLink,
    Link,
    Flat,
    Outlined,
    Filled,
}

/// A control that performs an action when triggered.
/// ```rust
/// use viewy::components::{Button, ButtonStyle};
/// Button::new("Label", ButtonStyle::Filled)
///     .action("/") // Here create a link to "/"
/// ```
#[derive(Debug, Clone)]
pub struct Button {
    node: Node,
    badge: Option<Badge>,
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub icon: Option<Box<dyn IconPack>>,
}

impl Button {
    /// Create new button
    pub fn new(label: &str, style: ButtonStyle) -> Self {
        let mut button = Button {
            node: Node::default(),
            badge: None,
            label: Some(label.to_string()),
            style,
            icon: None,
        };
        button.tag("button").set_attr("type", "button");
        button
    }

    /// Create new icon only button
    pub fn icon_only<T>(icon: T, style: ButtonStyle) -> Self
    where
        T: 'static + IconPack,
    {
        let mut button = Button {
            node: Node::default(),
            badge: None,
            label: None,
            style,
            icon: Some(Box::new(icon)),
        };
        button.tag("button").set_attr("type", "button");
        button
    }

    /// Change button style to destructive (red)
    pub fn destructive(&mut self) -> &mut Self {
        self.add_class(
            format!("button--{:?}--destructive", self.style)
                .to_lowercase()
                .as_str(),
        )
    }

    /// Disable interaction on the button
    pub fn disabled(&mut self, is_disabled: bool) -> &mut Self {
        if is_disabled {
            self.add_class(
                format!("button--{:?}--disabled", self.style)
                    .to_lowercase()
                    .as_str(),
            )
            .set_attr("disabled", "disabled")
        } else {
            self
        }
    }

    pub fn reversed(&mut self, is_reversed: bool) -> &mut Self {
        if is_reversed {
            self.add_class("button--reversed")
        } else {
            self
        }
    }

    /// Make the button submit specified form
    /// ```rust
    ///use viewy::components::{Appendable, Button, ButtonStyle, Form, View};
    /// View::new()
    ///    .append_child({
    ///        Form::new("formName", "/")
    ///    })
    ///    .append_child({
    ///        Button::new("Submit", ButtonStyle::Filled)
    ///            .attach_to_form("formName")
    ///        })
    /// ```
    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
        self.set_attr("form", form_name).set_attr("type", "submit")
    }

    pub fn attach_to_file_input(&mut self, input_id: &str) -> &mut Self {
        self.set_attr("data-input-file", &format!("file-input-{}", input_id))
    }

    pub fn close_popup(&mut self) -> &mut Self {
        self.add_class("popup__window-controls")
    }

    /// Set url to navigate to.
    pub fn action(&mut self, url: &str) -> &mut Self {
        self.set_attr("href", url).tag("a")
    }

    /// Set button's icon
    pub fn icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self
    }
}

impl std::ops::Deref for Button {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl DefaultModifiers for Button {}

impl BadgeSupport for Button {
    fn add_badge(&mut self, badge: Badge) {
        self.badge = Some(badge);
    }
}

impl BadgeModifiers for Button {}

impl Renderable for Button {
    fn render(mut self) -> Node {
        let button_style = format!("button--{:?}", self.style).to_lowercase();
        self.add_class("button")
            .add_class(&button_style)
            .set_attr("role", "button");

        if self.label.is_none() && self.icon.is_some() {
            self.add_class("button--icon-only");
        }

        if let Some(icon) = self.icon {
            let mut icon = Icon::new(icon).size(match self.style {
                ButtonStyle::SmallLink => 14,
                _ => 16,
            });
            if self.label.is_none() {
                icon.size(match self.style {
                    ButtonStyle::SmallLink => 16,
                    _ => 24,
                });
                icon.stroke_width(2);
            }
            self.node.children.push(icon.render());
        }

        if let Some(label) = self.label {
            let text = Text::new(
                label.as_str(),
                match self.style {
                    ButtonStyle::SmallLink => TextStyle::Caption,
                    _ => TextStyle::Button,
                },
            )
            .render();

            self.node.children.push(text);
        }

        if let Some(badge) = self.badge {
            self.node.children.push(badge.render());
        }

        self.node
    }
}
