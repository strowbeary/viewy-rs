use std::fmt::Debug;
use std::collections::HashMap;
use grass;
use crate::node::Node;

pub trait Renderable: Debug + RenderableClone {
    fn render(&self) -> Node;
}

pub trait RenderableClone {
    fn clone_box(&self) -> Box<dyn Renderable>;
}

impl<T> RenderableClone for T
    where
        T: 'static + Renderable + Clone,
{
    fn clone_box(&self) -> Box<dyn Renderable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Renderable> {
    fn clone(&self) -> Box<dyn Renderable> {
        self.clone_box()
    }
}

pub trait ToHtml : Renderable {
    fn to_html(&self) -> String {
        let root_node: Node = self.render();
        let popovers_html: Vec<String> = root_node.get_popovers().iter()
            .map(|popover| popover.render())
            .map(|node| node.get_html())
            .collect();

        format!("{view} {popover}", view = root_node.get_html(), popover = popovers_html.join(""))

    }
}