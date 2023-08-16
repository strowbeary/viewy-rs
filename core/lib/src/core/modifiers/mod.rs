use std::ops::DerefMut;
use uuid::Uuid;
use crate::components::popup::Popup;
use crate::core::node::Node;
use crate::core::widget::Widget;

pub trait Appendable: Widget {
    fn append_child<C>(&mut self, child: C) -> &mut Self
        where C: Into<Node>
     {
        let node: &mut Node = self.deref_mut();
        node.children.push(child.into());
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

pub trait Popupable: Widget + Classable + Attributable {
    fn popup(&mut self, mut popup: Popup) -> &mut Self {
        let id = Uuid::new_v4().to_string();
        self.add_class("popup--opener");
        self.set_attr("id", id.as_str());
        let node: &mut Node = self.deref_mut();
        node.root_nodes.insert(popup.attach_to(id.as_str()).into());
        self
    }
}