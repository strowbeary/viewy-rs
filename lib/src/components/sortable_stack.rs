use std::borrow::BorrowMut;
use std::ops::DerefMut;

use crate::components::icons::Lucide;
use crate::components::{Alignment, Appendable, HStack, Icon};
use crate::node::Node;
use crate::{DefaultModifiers, sp};
use crate::{Renderable, scale};

#[derive(Debug, Clone)]
pub struct SortableStack {
    node: Node,
    wrapper_tag: String,
    is_disabled: bool,
}
impl std::ops::Deref for SortableStack {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for SortableStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl DefaultModifiers for SortableStack {}

impl SortableStack {
    pub fn new() -> Self {
        SortableStack {
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
        self.node.children = self
            .node
            .children
            .into_iter()
            .map(|mut child| {
                if !self.is_disabled {
                    let mut container = HStack::new(Alignment::Center);
                    container
                        .tag(&self.wrapper_tag)
                        .add_class("sortable-stack__item")
                        .gap(vec![scale(3)])
                        .append_child(
                            Icon::new(Lucide::GripVertical)
                                .stroke_width(2)
                                .add_class("sortable-stack__item__handle")
                                .min_width(&sp(24)),
                        );

                    child.node_style.iter().for_each(|(property, value)| {
                        if property.eq("margin")
                            || property.eq("margin-top")
                            || property.eq("margin-right")
                            || property.eq("margin-left")
                            || property.eq("margin-bottom")
                        {
                            container
                                .deref_mut()
                                .node_style
                                .push((property.clone(), value.clone()))
                        }
                    });

                    child.node_style = child
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
                    container.deref_mut().children.push(child);
                    container.render()
                } else {
                    let mut stack = HStack::new(Alignment::Center);
                    stack
                        .tag(&self.wrapper_tag)
                        .add_class("sortable-stack__item")
                        .add_class("sortable-stack__item--disabled");
                    stack.deref_mut().children.push(child);
                    stack.render()
                }
            })
            .collect();

        self.node
    }
}
