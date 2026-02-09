use crate::core::node::Node;
use crate::core::widget::Widget;

mod actionnable;
mod box_stylable;
mod cardifiable;
mod sizable;

use crate::core::theme::Color;
#[doc(inline)]
pub use actionnable::*;
#[doc(inline)]
pub use box_stylable::*;
#[doc(inline)]
pub use cardifiable::*;

#[doc(inline)]
pub use sizable::*;

pub trait Appendable: Widget {
    fn append_child<C>(&mut self, child: C) -> &mut Self
    where
        C: Into<Node>,
    {
        let node: &mut Node = self.deref_mut();
        let child_node = child.into();

        node.children.push(child_node);

        self
    }

    fn set_children(&mut self, children: Vec<Node>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.children = children;
        self
    }
}

pub trait Classable: Widget {
    fn add_class(&mut self, class: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.class_list.insert(class.to_string());
        self
    }
    fn remove_class(&mut self, class: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.class_list.remove(class);
        self
    }
}
pub trait Attributable: Widget {
    fn set_attr(&mut self, name: &str, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.attributes.insert(name.to_string(), value.to_string());
        self
    }

    fn unset_attr(&mut self, name: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.attributes.remove(name);
        self
    }
}

pub trait Colorable: Widget {
    fn color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("color".to_string(), format!("var({})", color.as_str()));
        self
    }

    fn background_color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert(
            "background-color".to_string(),
            format!("var({})", color.as_str()),
        );
        self
    }
    fn border_color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert(
            "border-color".to_string(),
            format!("var({})", color.as_str()),
        );
        self
    }
}
