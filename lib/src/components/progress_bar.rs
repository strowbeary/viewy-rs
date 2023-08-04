use std::borrow::BorrowMut;

use crate::components::View;
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

/// Use this component to display the progress of a task
#[derive(Debug, Clone)]
pub struct ProgressBar {
    node: Node,

    /// The current numeric value. This must be between the minimum and maximum values (min attribute and max attribute) if they are specified. If unspecified or malformed, the value is 0. If specified, but not within the range given by the min attribute and max attribute, the value is equal to the nearest end of the range.
    /// > Note: Unless the value attribute is between 0 and 1 (inclusive), the min and max attributes should define the range so that the value attribute's value is within it.
    pub value: Option<f32>,

    /// The upper numeric bound of the measured range. This must be greater than the minimum value (min attribute), if specified. If unspecified, the maximum value is 1.0
    pub max: Option<f32>,

}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            node: Node::default(),
            value: None,
            max: None,
        }
    }

    pub fn value(&mut self, value: f32) -> Self {
        self.value = Some(value);
        self.clone()
    }

    pub fn max(&mut self, max: f32) -> Self {
        self.max = Some(max);
        self.clone()
    }
}


impl NodeContainer for ProgressBar {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<ProgressBar> for ProgressBar {}


impl Renderable for ProgressBar {
    fn render(&self) -> Node {
        let mut gauge = self.clone()
            .add_class("progress-bar");

        let mut progress_element = View::new()
            .tag("progress")
            .add_class("progress-bar__progress-element");

        if let Some(value) = &self.value {
            progress_element
                .set_attr("value", &value.to_string());
        }

        if let Some(max) = self.max {
            progress_element.set_attr("max", &max.to_string());
        }

        progress_element.node.text = Some(format!("{}/{}", gauge.value.unwrap_or(0.0), gauge.max.unwrap_or(1.0)));

        gauge.node.children.push(progress_element.render());

        gauge.node
    }
}