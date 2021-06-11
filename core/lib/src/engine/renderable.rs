use std::fmt::Debug;
use std::collections::HashMap;
use grass;
use crate::node::Node;
use std::ops::Deref;
use dyn_clone::{DynClone, clone_trait_object};

pub trait Renderable: DynClone + Debug {
    fn render(&self) -> Node;

    fn to_html(&self) -> String {
        let root_node: Node = self.render();
        let popovers_html: Vec<String> = root_node.get_popovers().iter()
            .map(|popover| popover.render())
            .map(|node| node.get_html())
            .collect();
        let popups_html: Vec<String> = root_node.get_popups().iter()
            .map(|popup| popup.render())
            .map(|node| node.get_html())
            .collect();
        format!(
            "{view} {popover} {popup}",
            view = root_node.get_html(),
            popover = popovers_html.join(""),
            popup = popups_html.join("")
        )
    }
}

clone_trait_object!(Renderable);

impl Renderable for Box<dyn Renderable> {
    fn render(&self) -> Node {
        self.deref().render()
    }
}
