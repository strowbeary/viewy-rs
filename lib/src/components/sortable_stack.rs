use std::borrow::BorrowMut;

use crate::components::icons::Lucide;
use crate::components::{Alignment, Appendable, ChildContainer, HStack, Icon};
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, sp};
use crate::{Renderable, scale};

#[derive(Debug, Clone)]
pub struct SortableStack {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    wrapper_tag: String,
    is_disabled: bool,
}

impl NodeContainer for SortableStack {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for SortableStack {}

impl SortableStack {
    pub fn new() -> Self {
        SortableStack {
            children: vec![],
            node: Default::default(),
            wrapper_tag: "div".to_string(),
            is_disabled: false,
        }
    }

    pub fn disabled(&mut self, is_disabled: bool) -> &mut Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn wrapper_tag(&mut self, tag_name: &str) -> &mut Self {
        self.wrapper_tag = tag_name.to_string();
        self
    }

    /// Define url of the post request to send order changes requests
    pub fn action(&mut self, action: &str) -> &mut Self {
        self.set_attr("data-action", action)
    }

    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node
            .node_style
            .push(("grid-gap".to_string(), params.join(" ")));
        self
    }
}

impl ChildContainer for SortableStack {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for SortableStack {}

impl Renderable for SortableStack {
    fn render(mut self) -> Node {
        self.add_class("stack")
            .add_class("stack--vertical")
            .add_class("stack--align-stretch")
            .add_class("sortable-stack");
        if self.is_disabled {
            self.add_class("sortable-stack--disabled");
        }
        self.children.into_iter().for_each(|child| {
            if !self.is_disabled {
                self.node.children.push({
                    let mut child_node = child.render();
                    let mut container = HStack::new(Alignment::Center);
                    container
                        .tag(&self.wrapper_tag)
                        .add_class("sortable-stack__item")
                        .gap(vec![scale(3)])
                        .append_child({
                            Icon::new(Lucide::GripVertical)
                                .stroke_width(2)
                                .add_class("sortable-stack__item__handle")
                                .min_width(&sp(24))
                        });

                    child_node.node_style.iter().for_each(|(property, value)| {
                        if property.eq("margin")
                            || property.eq("margin-top")
                            || property.eq("margin-right")
                            || property.eq("margin-left")
                            || property.eq("margin-bottom")
                        {
                            container
                                .get_node()
                                .node_style
                                .push((property.clone(), value.clone()))
                        }
                    });

                    child_node.node_style = child_node
                        .node_style
                        .into_iter()
                        .filter(|(property, _)| {
                            !(property.eq("margin")
                                || property.eq("margin-top")
                                || property.eq("margin-right")
                                || property.eq("margin-left")
                                || property.eq("margin-bottom"))
                        })
                        .collect();
                    container.get_node().children.push(child_node);
                    container.render()
                })
            } else {
                self.node.children.push({
                    let mut stack = HStack::new(Alignment::Center);
                    stack
                        .tag(&self.wrapper_tag)
                        .add_class("sortable-stack__item")
                        .add_class("sortable-stack__item--disabled")
                        .append_child(child);
                    stack.render()
                })
            }
        });

        self.node
    }
}
