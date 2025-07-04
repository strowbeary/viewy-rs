use crate::core::modifiers::{Attributable, Classable};
use crate::core::node::{Node, NodeType};
use crate::core::widget::Widget;
use crate::modifiers::PopupReceiver;
use crate::prelude::actionnable::OnClickActionnable;

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
}
impl PopupReceiver for Button {}

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
        }
    }

    /// Reverse the icon and text
    #[deprecated(
        since = "2.0.0",
        note = "please use `reverse` instead. Will be removed in 2.1"
    )]
    pub fn reversed(&mut self, is_reversed: bool) -> &mut Self {
        if is_reversed {
            self.add_class("button--reversed")
        } else {
            self
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

    /// Make the button close the popup it is in
    /// ```rust
    /// Popup::new()
    ///    .append_child({
    ///        Button::new("Submit", ButtonStyle::Filled)
    ///            .close_popup()
    ///     })
    /// ```
    pub fn close_popup(&mut self) -> &mut Self {
        self.add_class("popup__window-controls")
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
    pub fn attach_to_form(&mut self, form_name: &str) -> &mut Self {
        self.set_attr("form", form_name).set_attr("type", "submit")
    }

    /// Make the button submit specified form
    /// ```rust
    ///View::new()
    ///    .append_child({
    ///        FileInput::new("input-name", FileInputType::Hidden)
    ///    })
    ///    .append_child({
    ///        Button::new("Submit", ButtonStyle::Filled)
    ///            .attach_to_file_input("input-name")
    ///     })
    /// ```
    pub fn attach_to_file_input(&mut self, input_id: &str) -> &mut Self {
        self.set_attr("data-input-file", &format!("file-input-{}", input_id))
    }

    fn render(&mut self) {
        let style = self.style.clone();
        self.add_class("button")
            .add_class(format!("button--{:?}", style).to_lowercase().as_str());
        self.node.text = self.label.clone();
    }
}

impl OnClickActionnable for Button {
    fn on_click(&mut self, action: crate::prelude::actionnable::Action) -> &mut Self {
        match &action {
            crate::prelude::actionnable::Action::Navigate { url } => {
                self.node.node_type = NodeType::Normal("a");
                self.set_attr("href", url);
            }

            _ => {
                action.apply("click", self);
            }
        }
        self
    }
}
