use crate::bindings::uri::Uri;
use crate::modifiers::{Appendable, Attributable, Classable, Cardifiable};
use crate::core::node::Node;
use crate::core::widget::Widget;
use crate::node::NodeType;
use rocket::uri;

pub enum FormMethod {
    Get,
    Post,
}

/// A control that performs an action when triggered.
/// ```rust
/// use viewy::prelude::*;
/// Button::new("Label", ButtonStyle::Filled)
///     .action("/") // Here create a link to "/"
/// ```
#[derive(Widget, Classable, Attributable, Appendable, Cardifiable)]
#[widget(style = "./style.scss")]
pub struct Form {
    node: Node,
    uri: Uri,
    method: FormMethod,
}

impl Form {
    pub fn new(method: FormMethod, uri: impl Into<Uri>) -> Self {
        Form {
            node: Node {
                node_type: NodeType::Normal("form"),
                ..Node::default()
            },
            uri: uri.into(),
            method,
        }
    }

    pub fn render(&mut self) {
        match self.method {
            FormMethod::Get => {} // no `method` attribute will use GET as request method following standard browser behavior
            FormMethod::Post => {
                self.set_attr("method", "POST");
            }
        }
        self.add_class("form");
        self.set_attr("action", &self.uri.to_string());
    }
}
