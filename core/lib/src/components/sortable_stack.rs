use crate::{Renderable, scale};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp};
use crate::components::{Alignment, Appendable, ChildContainer, HStack, Icon, View};

#[derive(Debug, Clone)]
pub struct SortableStack {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    wrapper_tag: String,
}

impl NodeContainer for SortableStack {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<SortableStack> for SortableStack {}

impl SortableStack {
    pub fn new() -> Self {
        SortableStack {
            children: vec![],
            node: Default::default(),
            wrapper_tag: "div".to_string()
        }
    }

    pub fn wrapper_tag(&mut self, tag_name: &str) -> Self {
        self.wrapper_tag = tag_name.to_string();
        self.clone()
    }

    pub fn action(&mut self, action: &str) -> Self {
        self.set_attr("data-action", action)
    }

    pub fn gap(&mut self, gaps: Vec<i32>) -> Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self.clone()
    }

}
impl ChildContainer for SortableStack {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for SortableStack {}
impl Renderable for SortableStack {
    fn render(&self) -> Node {
        let mut stack = self
            .clone()
            .add_class("stack")
            .add_class("sortable-stack")
            .add_class("stack--vertical")
            .add_class("stack--align-stretch");
        stack.children.into_iter()
            .for_each(|child|
                stack.node.children.push({
                    HStack::new(Alignment::Center)
                        .tag(&self.wrapper_tag)
                        .add_class("sortable-stack__item")
                        .gap(vec![scale(3)])
                        .append_child({
                            Icon::new("align-justify")
                                .stroke_width(2)
                                .add_class("sortable-stack__item__handle")
                        })
                        .append_child(child)
                        .render()
                }));
        stack.node
    }
}