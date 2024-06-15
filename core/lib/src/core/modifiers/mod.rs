use rayon::prelude::IntoParallelIterator;
use rocket::futures::StreamExt;
use uuid::Uuid;
use crate::widgets::popup::Popup;
use crate::core::node::Node;
use crate::core::widget::Widget;

pub mod box_stylable;

#[doc(inline)]
pub use box_stylable::*;
use crate::core::theme::Color;

pub trait Appendable: Widget {
    fn append_child<C>(&mut self, child: C) -> &mut Self
        where C: Into<Node>
     {
        let node: &mut Node = self.deref_mut();
        node.children.push(child.into());
        self
    }

    fn set_children(&mut self, children: Vec<Node>) -> &mut Self
    {
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

pub trait PopupReceiver: Widget + Classable + Attributable {
    fn popup<P>(&mut self, mut popup: P) -> &mut Self where P: AsMut<Popup> {
        let popup = popup.as_mut();
        let id = Uuid::new_v4().to_string();
        self.add_class("popup--opener");
        self.set_attr("id", id.as_str());
        let node: &mut Node = self.deref_mut();
        node.root_nodes.insert(popup.attach_to(id.as_str()).into());
        self
    }
}


pub trait Colorable: Widget {
   fn color(&mut self, color: Color) -> &mut Self {
       let node: &mut Node = self.deref_mut();
       node.node_style.push(("color".to_string(), format!("var({})", color.as_str())));
       self
   }

    fn background_color(&mut self, color: Color) -> &mut Self {

        let node: &mut Node = self.deref_mut();
        node.node_style.push(("background-color".to_string(), format!("var({})", color.as_str())));
        self
    }
    fn border_color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.push(("border-color".to_string(), format!("var({})", color.as_str())));
        self
    }
}
