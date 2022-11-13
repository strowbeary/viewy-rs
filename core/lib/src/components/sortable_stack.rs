use std::borrow::BorrowMut;

use crate::{Renderable, scale};
use crate::{DefaultModifiers, sp};
use crate::components::{Alignment, Appendable, ChildContainer, HStack, Icon, View};
use crate::components::icons::Lucide;
use crate::node::{Node, NodeContainer};

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

impl DefaultModifiers<SortableStack> for SortableStack {}

impl SortableStack {
    pub fn new() -> Self {
        SortableStack {
            children: vec![],
            node: Default::default(),
            wrapper_tag: "div".to_string(),
            is_disabled: false,
        }
    }

    pub fn disabled(&mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self.clone()
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
            .add_class("stack--vertical")
            .add_class("stack--align-stretch")
            .add_class("sortable-stack");
        if self.is_disabled {
            stack.add_class("sortable-stack--disabled");
        }
        stack.children.into_iter()
            .for_each(|child| {
                if !self.is_disabled {
                    stack.node.children.push({
                        let mut child_node = child.render();
                        let mut container_node = HStack::new(Alignment::Center)
                            .tag(&self.wrapper_tag)
                            .add_class("sortable-stack__item")
                            .gap(vec![scale(3)])
                            .append_child({
                                Icon::new(Lucide::AlignJustify)
                                    .stroke_width(2)
                                    .add_class("sortable-stack__item__handle")
                                    .min_width(&sp(24))
                            })
                            .render();


                        child_node.node_style.iter()
                            .for_each(|(property, value)| {
                                if property.eq("margin")
                                    || property.eq("margin-top")
                                    || property.eq("margin-right")
                                    || property.eq("margin-left")
                                    || property.eq("margin-bottom") {
                                    container_node.node_style.push((property.clone(), value.clone()))
                                }
                            });

                    child_node.node_style = child_node.node_style.into_iter()
                        .filter(|(property, value)| {
                            !(property.eq("margin")
                                || property.eq("margin-top")
                                || property.eq("margin-right")
                                || property.eq("margin-left")
                                || property.eq("margin-bottom"))
                        })
                        .collect();
                    container_node.children.push(child_node);
                    container_node
                })
            } else {
                          stack.node.children.push({
                              HStack::new(Alignment::Center)
                                  .tag(&self.wrapper_tag)
                                  .add_class("sortable-stack__item")
                                  .add_class("sortable-stack__item--disabled")
                                  .append_child(child)
                                  .render()
                          })
                      }
    });

    stack.node
}
}