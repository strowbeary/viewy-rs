use rocket::http::uri::Reference;

use crate::core::modifiers::{Appendable, Attributable, Classable};
use crate::core::node::Node;
use crate::core::widget::Widget;
use crate::node::NodeType;

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
#[derive(Widget, Classable, Attributable, Appendable)]
#[widget(style = "./style.scss")]
pub struct Form {
    node: Node,
    uri: Option<Reference<'static>>,
    method: FormMethod,
}

impl Form {
    pub fn new<U: TryInto<Reference<'static>>>(method: FormMethod, uri: U) -> Self {
        Form {
            node: Node {
                node_type: NodeType::Normal("form"),
                ..Node::default()
            },
            uri: uri.try_into().ok(),
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
        self.set_attr(
            "action",
            &match &self.uri {
                Some(uri) => uri.to_string(),
                None => String::new(),
            },
        );
    }
}
