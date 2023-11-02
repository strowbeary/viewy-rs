use std::ops::{Deref, DerefMut};

use super::node::Node;

pub trait Widget: Deref<Target=Node> + DerefMut<Target=Node> + Into<Node> {
    const STYLE: &'static str;
    fn widget_name() -> &'static str;


    /// render the widget in HTML by consuming Node
    fn to_html(self) -> String {
        let node: Node = self.into();
        let root_nodes = node.get_root_nodes().into_iter()
            .collect::<Vec<String>>()
            .join("");
        let view: String = node.into();
        format!(
            "{view} {root_nodes}",
        )
    }
}



