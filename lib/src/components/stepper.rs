use std::borrow::BorrowMut;

use crate::components::{Appendable, ChildContainer};
use crate::components::icons::IconPack;
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

/// An outlined view to emphasize a content.
#[derive(Debug, Clone)]
pub struct Step {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    name: String,
    icon: Option<Box<dyn IconPack>>,
}

impl Step {
    pub fn new(name: &str) -> Self {
        Self {
            children: vec![],
            node: Default::default(),
            name: name.to_string(),
            icon: None,
        }
    }

    pub fn icon(&mut self, icon: Box<dyn IconPack>) -> Self {
        self.icon = Some(icon);
        self.clone()
    }
}

impl NodeContainer for Step {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Step> for Step {}

impl ChildContainer for Step {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Step {}

impl Renderable for Step {
    fn render(&self) -> Node {
        let mut view = self.clone()
            .add_class("card")
            .add_class(format!("card--{:?}", self.style).to_lowercase().as_str())
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view
    }
}


/// Use a `Stepper` when you need to display progress state of something
/// ```rust
/// Stepper::new()
///     .selected(2)
///     .append_child(Step::new("toto 1"))
///     .append_child(Step::new("toto 2"))
///     .append_child(Step::new("toto 3")) // this one will be in selected state
/// ```
#[derive(Debug, Clone)]
pub struct Stepper {
    children: Vec<Box<dyn Renderable>>,
    steps: Vec<Step>,
    selected_step: usize,
    node: Node,
}

impl Stepper {
    pub fn new() -> Self {
        Stepper {
            children: vec![],
            steps: vec![],
            selected_step: 0,
            node: Node::default(),
        }
    }

    pub fn append_child(&mut self, child: Step) -> Self {
        self.steps.push(child);
        self.clone()
    }

    pub fn selected(&mut self, index: usize) -> Self {
        self.selected_step = index;
        self.clone()
    }
}


impl NodeContainer for Stepper {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Stepper> for Stepper {}


impl Renderable for Stepper {
    fn render(&self) -> Node {
        let mut view = self.clone()
            .add_class("card")
            .add_class(format!("card--{:?}", self.style).to_lowercase().as_str())
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view
    }
}