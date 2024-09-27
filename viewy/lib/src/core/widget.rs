use std::ops::{Deref, DerefMut};
use crate::node::HtmlCssJs;
use super::node::Node;

pub trait Widget: Deref<Target=Node> + DerefMut<Target=Node> + Into<Node> {
    const STYLE: &'static str;
    const SCRIPT: &'static str;
    fn widget_name() -> &'static str;


    /// render the widget in HTML by consuming Node
    fn to_html(self) -> String {
        let node: Node = self.into();
        let root_nodes = node.get_root_nodes().into_iter()
            .collect::<Vec<HtmlCssJs>>()
            .into_iter().map(|HtmlCssJs{html, ..}| html)
            .collect::<Vec<String>>()
            .join("");
        let view: HtmlCssJs = node.into();
        format!(
            "{} {root_nodes}",
            view.html
        )
    }
}



