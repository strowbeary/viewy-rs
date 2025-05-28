use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::Appendable;
use crate::node::Node;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum FormMethod {
    Post,
    Get,
}

#[derive(Debug, Clone)]
pub struct Form {
    node: Node,
    pub name: String,
    pub action: String,
    pub is_async: bool,
    pub form_method: FormMethod,
}

impl DefaultModifiers for Form {}

impl Form {
    pub fn new(name: &str, action: &str) -> Self {
        Form {
            node: Node::default(),
            name: name.to_string(),
            action: action.to_string(),
            is_async: false,
            form_method: FormMethod::Post,
        }
    }
    pub fn method(&mut self, method: FormMethod) -> &mut Self {
        self.form_method = method;
        self
    }
    pub fn async_form(&mut self) -> &mut Self {
        self.is_async = true;
        self
    }

    pub fn inject_into_dynamic_content(&mut self, dynamic_content_name: &str) -> &mut Self {
        self.set_attr("data-dynamic-content-name", dynamic_content_name)
    }

    pub fn multipart(&mut self) -> &mut Self {
        self.set_attr("enctype", "multipart/form-data")
    }
}
impl std::ops::Deref for Form {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Form {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl Appendable for Form {}

impl Renderable for Form {
    fn render(mut self) -> Node {
        let method = match self.form_method {
            FormMethod::Post => "POST",
            FormMethod::Get => "GET",
        };
        let name = self.name.to_string();
        let action = self.action.to_string();
        self.set_attr("id", &name)
            .add_class("form")
            .set_attr("action", &action)
            .tag("form")
            .set_attr("method", method);

        if self.is_async {
            self.set_attr("data-async", "data-async");
        }

        self.node
    }
}
