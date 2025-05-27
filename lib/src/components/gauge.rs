use std::borrow::BorrowMut;

use crate::components::View;
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

/// Used to set card style.
#[derive(Debug, Clone)]
pub enum GaugeStyle {
    Bar,
    Radial,
}

/// An outlined view to emphasize a content.
#[derive(Debug, Clone)]
pub struct Gauge {
    node: Node,

    /// The current numeric value. This must be between the minimum and maximum values (min attribute and max attribute) if they are specified. If unspecified or malformed, the value is 0. If specified, but not within the range given by the min attribute and max attribute, the value is equal to the nearest end of the range.
    /// > Note: Unless the value attribute is between 0 and 1 (inclusive), the min and max attributes should define the range so that the value attribute's value is within it.
    pub value: f32,

    /// The lower numeric bound of the measured range. This must be less than the maximum value (max attribute), if specified. If unspecified, the minimum value is 0.0
    pub min: Option<f32>,

    /// The upper numeric bound of the measured range. This must be greater than the minimum value (min attribute), if specified. If unspecified, the maximum value is 1.0
    pub max: Option<f32>,

    /// The upper numeric bound of the low end of the measured range. This must be greater than the minimum value (min attribute), and it also must be less than the high value and maximum value (high attribute and max attribute, respectively), if any are specified. If unspecified, or if less than the minimum value, the low value is equal to the minimum value.
    pub low: Option<f32>,

    /// The lower numeric bound of the high end of the measured range. This must be less than the maximum value (max attribute), and it also must be greater than the low value and minimum value (low attribute and min attribute, respectively), if any are specified. If unspecified, or if greater than the maximum value, the high value is equal to the maximum value.
    pub high: Option<f32>,

    /// This attribute indicates the optimal numeric value. It must be within the range (as defined by the min attribute and max attribute). When used with the low attribute and high attribute, it gives an indication where along the range is considered preferable. For example, if it is between the min attribute and the low attribute, then the lower range is considered preferred. The browser may color the meter's bar differently depending on whether the value is less than or equal to the optimum value.
    pub optimum: Option<f32>,

    /// This attribute determine if the optimum indicator is visible or not.
    pub display_optimum_indicator: bool,
    pub style: GaugeStyle,
}

impl Gauge {
    pub fn new(value: f32, style: GaugeStyle) -> Self {
        Gauge {
            node: Node::default(),
            value,
            min: None,
            max: None,
            low: None,
            high: None,
            optimum: None,
            display_optimum_indicator: false,
            style,
        }
    }

    pub fn min(&mut self, min: f32) -> &mut Self {
        self.min = Some(min);
        self
    }

    pub fn max(&mut self, max: f32) -> &mut Self {
        self.max = Some(max);
        self
    }

    pub fn low(&mut self, low: f32) -> &mut Self {
        self.low = Some(low);
        self
    }

    pub fn high(&mut self, high: f32) -> &mut Self {
        self.high = Some(high);
        self
    }

    pub fn optimum(&mut self, optimum: f32) -> &mut Self {
        self.optimum = Some(optimum);
        self
    }

    pub fn display_optimum_indicator(&mut self) -> &mut Self {
        self.display_optimum_indicator = true;
        self
    }
}


impl NodeContainer for Gauge {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Gauge {}


impl Renderable for Gauge {
    fn render(mut self) -> Node {
        let gauge_style = format!("gauge--{:?}", self.style).to_lowercase();
        self
            .add_class("gauge")
            .add_class(&gauge_style);

        if self.display_optimum_indicator {
            self.add_class("gauge__optimum-indicator");
        }

        let mut meter_element = View::new();
        meter_element.tag("meter")
            .add_class("gauge__meter-bar")
            .set_attr("value", &self.value.to_string());


        if let Some(min) = self.min {
            meter_element.set_attr("min", &min.to_string());
        }
        if let Some(max) = self.max {
            meter_element.set_attr("max", &max.to_string());
        }
        if let Some(low) = self.low {
            meter_element.set_attr("low", &low.to_string());
        }
        if let Some(high) = self.high {
            meter_element.set_attr("high", &high.to_string());
        }
        if let Some(optimum) = self.optimum {
            meter_element.set_attr("optimum", &optimum.to_string());
        }

        meter_element.node.text = Some(format!("{}/{}", self.value, self.max.unwrap_or(1.0)));

        self.node.children.push(meter_element.render());

        self.node
    }
}