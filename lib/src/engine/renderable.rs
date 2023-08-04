use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap};
use grass;
use crate::node::Node;
use std::ops::Deref;
use dyn_clone::{DynClone, clone_trait_object};

pub trait Renderable: DynClone + Send + Sync + Debug {
    fn render(&self) -> Node;

    fn component_name(&self) -> &str {
        "View"
    }

    fn to_html(&self) -> String {
        let current_view: Node = self.render();
        let root_nodes_html: Vec<String> = current_view.get_root_nodes().iter()
            .map(|root_node| root_node.to_html())
            .collect();
        format!(
            "{view} {root_nodes}",
            view = current_view.get_html(),
            root_nodes = root_nodes_html.join(""),
        )
    }
}

clone_trait_object!(Renderable);

impl Renderable for Box<dyn Renderable> {
    fn render(&self) -> Node {
        self.deref().render()
    }
}

impl PartialEq for Box<dyn Renderable> {
    fn eq(&self, other: &Box<dyn Renderable>) -> bool {
        self.deref().component_name().eq(other.deref().component_name())
    }
}
impl Eq for Box<dyn Renderable> {

}


impl Hash for Box<dyn Renderable> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().component_name().hash(state);
    }
}