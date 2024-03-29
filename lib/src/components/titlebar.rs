use std::borrow::BorrowMut;

use crate::components::{Alignment, Appendable, Text, TextStyle, VStack};
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

#[derive(Debug, Clone)]
pub struct TitleBar {
    node: Node,
    pub title: String,
    pub subtitle: Option<String>,
    pub is_sticky: bool,
    pub left_item: Option<Box<dyn Renderable>>,
    pub right_item: Option<Box<dyn Renderable>>,
    pub bottom_item: Option<Box<dyn Renderable>>,
}

impl NodeContainer for TitleBar {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<TitleBar> for TitleBar {}

impl TitleBar {
    pub fn new(title: &str) -> Self {
        TitleBar {
            node: Default::default(),
            title: title.to_string(),
            subtitle: None,
            is_sticky: true,
            left_item: None,
            right_item: None,
            bottom_item: None,
        }
    }

    pub fn subtitle(&mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self.clone()
    }
    fn grid_areas(&mut self, schema: &str) -> Self {
        self.node.node_style.push(("grid-template-areas".to_string(), schema.to_string()));
        self.clone()
    }

    pub fn sticky(&mut self, is_sticky: bool) -> Self {
        self.is_sticky = is_sticky;
        self.clone()
    }

    pub fn left_item<'a, T>(&'a mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.left_item = Some(Box::new(item));
        self.clone()
    }
    pub fn right_item<'a, T>(&'a mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.right_item = Some(Box::new(item));
        self.clone()
    }
    pub fn bottom_item<'a, T>(&'a mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.bottom_item = Some(Box::new(item));
        self.clone()
    }
}

impl Renderable for TitleBar {
    fn render(&self) -> Node {
        let mut areas = String::new();
        if self.left_item.is_some() {
            areas.push_str("'left_item . right_item' 'title title title'");
        } else {
            areas.push_str("'title title right_item'");
        }
        if self.bottom_item.is_some() {
            areas.push_str("'bottom_item bottom_item bottom_item'");
        }
        let mut view = self.clone()
            .add_class("titlebar")
            .grid_areas(areas.as_str());

        if self.is_sticky {
            view.sticky_to_top(0);
        }

        view.node.children.push({
            if let Some(subtitle) = &self.subtitle {
                VStack::new(Alignment::Stretch)
                    .grid_area("title")
                    .append_child({
                        Text::new(self.title.as_str(), TextStyle::LargeTitle)
                    })
                    .append_child({
                        Text::new(subtitle, TextStyle::H3)
                    })
                    .render()
            } else {
                Text::new(self.title.as_str(), TextStyle::LargeTitle)
                    .grid_area("title")
                    .render()
            }
        });
        if let Some(left_item) = self.left_item.clone() {
            let mut item = left_item.render();
            item.node_style.push(("grid-area".to_string(), "left_item".to_string()));
            view.node.children.push(item);
        }
        if let Some(right_item) = self.right_item.clone() {
            let mut item = right_item.render();
            item.node_style.push(("grid-area".to_string(), "right_item".to_string()));
            view.node.children.push(item);
        }
        if let Some(bottom_item) = self.bottom_item.clone() {
            let mut item = bottom_item.render();
            item.node_style.push(("grid-area".to_string(), "bottom_item".to_string()));
            view.node.children.push(item);
        }
        view.node
    }
}