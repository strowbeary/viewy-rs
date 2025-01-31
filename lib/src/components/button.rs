use std::borrow::BorrowMut;

use crate::components::badge::{Badge, BadgeSupport};
use crate::components::icons::IconPack;
use crate::components::{BadgeModifiers, Icon, Text, TextStyle};
use crate::node::{Node, NodeContainer};
use crate::Renderable;
use crate::DefaultModifiers;

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
/// Button::new("Label", ButtonStyle::Filled)
///     .action("/") // Here create a link to "/"
/// ```
#[derive(Debug, Clone)]
pub struct Button {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    badge: Option<Badge>,
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub icon: Option<Box<dyn IconPack>>,
}

impl Button {
    /// Create new button
    pub fn new(label: &str, style: ButtonStyle) -> Self {
        Button {
            children: vec![],
            node: Node::default(),
            badge: None,
            label: Some(label.to_string()),
            style,
            icon: None,
        }
            .tag("button")
            .set_attr("type", "button")
    }


    /// Create new icon only button
    pub fn icon_only<T>(icon: T, style: ButtonStyle) -> Self
    where
        T: 'static + IconPack,
    {
        Button {
            children: vec![],
            node: Node::default(),
            badge: None,
            label: None,
            style,
            icon: Some(Box::new(icon)),
        }
            .tag("button")
            .set_attr("type", "button")
    }

    /// Change button style to destructive (red)
    pub fn destructive(&mut self) -> Self {
        self.add_class(format!("button--{:?}--destructive", self.style).to_lowercase().as_str())
    }

    /// Disable interaction on the button
    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        if is_disabled {
            self.add_class(format!("button--{:?}--disabled", self.style).to_lowercase().as_str());
            self.set_attr("disabled", "disabled")
        } else {
            self.clone()
        }
    }

    pub fn reversed(&mut self, is_reversed: bool) -> Self {
        if is_reversed {
            self.add_class("button--reversed")
        } else {
            self.clone()
        }
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
        self.set_attr("form", form_name);
        self.set_attr("type", "submit")
    }

    pub fn attach_to_file_input(&mut self, input_id: &str) -> Self {
        self
            .set_attr("data-input-file", &format!("file-input-{}", input_id))
    }

    pub fn close_popup(&mut self) -> Self {
        self.add_class("popup__window-controls")
    }

    /// Set url to navigate to.
    pub fn action(&mut self, url: &str) -> Self {
        self.set_attr("href", url);
        self.tag("a")
    }

    /// Set button's icon
    pub fn icon<T>(&mut self, icon: T) -> Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self.clone()
    }

    fn append_child<'a, T>(&'a mut self, child: T)
    where
        T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }
}

impl NodeContainer for Button {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Button> for Button {}

impl BadgeSupport for Button {
    fn add_badge(&mut self, badge: Badge) {
        self.badge = Some(badge);
    }
}

impl BadgeModifiers for Button {}

impl Renderable for Button {
    fn render(&self) -> Node {
        let mut button = self.clone()
            .add_class("button")
            .add_class(format!("button--{:?}", self.style).to_lowercase().as_str())
            .set_attr("role", "button");

        if button.label.is_none() && button.icon.is_some() {
            button = button.add_class("button--icon-only");
        }

        if let Some(icon) = button.icon {
            let mut icon = Icon::new(icon)
                .size(match self.style {
                    ButtonStyle::SmallLink => 14,
                    _ => 16
                });
            if button.label.is_none() {
                icon.size(match self.style {
                    ButtonStyle::SmallLink => 16,
                    _ => 24
                });
                icon.stroke_width(2);
            }
            button.node.children.push(icon.render());
        }

        if let Some(label) = button.label {
            let text = Text::new(label.as_str(), match self.style {
                ButtonStyle::SmallLink => TextStyle::Caption,
                _ => TextStyle::Button
            }).render();

            button.node.children.push(text);
        }

        if let Some(badge) = button.badge {
            button.node.children.push(badge.render());
        }

        button.node
    }
}