use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, scale};
use crate::components::{Icon, Text, TextStyle};

/// Used to set a button's importance level.
#[derive(Debug, Clone)]
pub enum ButtonStyle {
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
    pub label: String,
    pub style: ButtonStyle,
    pub icon: Option<String>,
}

impl Button {
    /// Create new button
    pub fn new(label: &str, style: ButtonStyle) -> Self {
        Button {
            children: vec![],
            node: Node::default(),
            label: label.to_string(),
            style,
            icon: None,
        }
            .tag("button")
    }

    /// Change button style to destructive (red)
    pub fn destructive(&mut self) -> Self {
        self.add_class(format!("button--{:?}--destructive", self.style).to_lowercase().as_str())
    }

    /// Disable interaction on the button
    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        if is_disabled {
            self.add_class(format!("button--{:?}--disabled", self.style).to_lowercase().as_str())
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
        self
            .set_attr("form", form_name)
            .set_attr("type", "submit")
    }

    /// Set url to navigate to.
    pub fn action(&mut self, url: &str) -> Self {
        self
            .set_attr("href", url)
            .tag("a")
    }

    /// Set button's icon
    pub fn icon(&mut self, name: &str) -> Self {
        self.icon = Some(name.to_string());
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

impl Renderable for Button {
    fn render(&self) -> Node {
        let mut button = self.clone()
            .add_class("button")
            .add_class(format!("button--{:?}", self.style).to_lowercase().as_str())
            .set_attr("role", "button");

        if let Some(icon) = button.icon {
            let icon = Icon::new(icon.as_str())
                .size(16)
                .render();
            button.node.children.push(icon);
        }
        let text = Text::new(self.label.as_str(), TextStyle::Button).render();
        button.node.children.push(text);
        button.node
    }
}