use crate::core::node::{Node, NodeType};
use crate::core::widget::Widget;
use crate::modifiers::{Attributable, Classable, OnClickActionnable};
use crate::prelude::{Icon, IconPack, Text, TextStyle};

/// Used to set a button's importance level.
#[derive(Debug, Clone)]
pub enum ButtonStyle {
    Link,
    Flat,
    Outlined,
    Filled,
    SmallLink,
}

/// A control that performs an action when triggered.
/// ```rust
/// use viewy::prelude::*;
/// Button::new("Label", ButtonStyle::Filled)
///     .action("/") // Here create a link to "/"
/// ```
#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct Button {
    node: Node,
    /// Button label, If `None` it's an icon only button
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub icon: Option<Box<dyn IconPack>>,
}

impl Button {
    pub fn new(label: &str, style: ButtonStyle) -> Self {
        Button {
            node: {
                let mut node = Node::default();
                node.node_type = NodeType::Normal("button");
                node
            },
            label: Some(label.to_string()),
            style,
            icon: None,
        }
    }

    /// Reverse the icon and text
    pub fn reverse(&mut self) -> &mut Self {
        self.add_class("button--reversed")
    }

    /// Turn the button to the destructive theme variant for destructive actions
    /// ```rust
    /// Button::new("Destructive action", ButtonStyle::Filled)
    ///    .destructive()
    /// ```
    pub fn destructive(&mut self) -> &mut Self {
        self.add_class(
            format!("button--{:?}--destructive", self.style)
                .to_lowercase()
                .as_str(),
        )
    }

    /// Make the button as disabled
    /// ```rust
    /// Button::new("Disabled action", ButtonStyle::Filled)
    ///    .disabled()
    /// ```
    pub fn disabled(&mut self) -> &mut Self {
        self.set_attr("disabled", "disabled")
    }

    /// Set button's icon
    pub fn icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self
    }

    fn render(&mut self) {
        let style = self.style.clone();
        self.add_class("button")
            .add_class(format!("button--{:?}", style).to_lowercase().as_str());
        if let Some(icon_from_pack) = self.icon.clone() {
            let mut icon = Icon::new(icon_from_pack);
            icon.size(match self.style {
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
            self.node.children.push(icon.into());
        }
        if let Some(label) = &self.label {
            let mut text_node: Node = Text::new(label, TextStyle::Body).into();
            text_node.node_type = NodeType::Normal("span");
            text_node.class_list.clear();
            self.node.children.push(text_node);
        }
    }
}

impl OnClickActionnable for Button {
    fn on_click(&mut self, action: crate::prelude::Action) -> &mut Self {
        action.apply("click", self);

        self
    }
}
