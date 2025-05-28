use std::borrow::BorrowMut;

use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::Appendable;
use crate::components::icons::IconPack;
use crate::node::Node;

/// An outlined view to emphasize a content.
#[derive(Debug, Clone)]
pub struct Step {
    node: Node,
    name: String,
    icon: Option<Box<dyn IconPack>>,
}

impl Step {
    pub fn new(name: &str) -> Self {
        Self {
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

impl std::ops::Deref for Step {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Step {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for Step {}

impl Appendable for Step {}

impl Renderable for Step {
    fn render(mut self) -> Node {
        self.add_class("card");
        self.node
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
    steps: Vec<Step>,
    selected_step: usize,
    node: Node,
}

impl Stepper {
    pub fn new() -> Self {
        Stepper {
            steps: vec![],
            selected_step: 0,
            node: Node::default(),
        }
    }

    pub fn append_child(&mut self, child: Step) -> &mut Self {
        self.steps.push(child);
        self
    }

    pub fn selected(&mut self, index: usize) -> &mut Self {
        self.selected_step = index;
        self
    }
}

impl std::ops::Deref for Stepper {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Stepper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl DefaultModifiers for Stepper {}

impl Renderable for Stepper {
    fn render(mut self) -> Node {
        self.add_class("card");
        self.node
    }
}
