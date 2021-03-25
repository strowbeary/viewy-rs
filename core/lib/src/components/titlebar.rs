use crate::node::{Node, NodeContainer};
use crate::renderer::{Renderable, ToHtml};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::components::{TextStyle, Text, View};

#[derive(Debug, Clone)]
pub struct TitleBar {
    view: Node,
    pub title: String,
    pub left_item: Option<Box<dyn Renderable>>,
    pub right_item: Option<Box<dyn Renderable>>,
    pub bottom_item: Option<Box<dyn Renderable>>,
}

impl NodeContainer for TitleBar {
    fn get_node(&mut self) -> &mut Node {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<TitleBar> for TitleBar {}

impl ToHtml for TitleBar {}

impl TitleBar {
    pub fn new(title: &str) -> Self {
        TitleBar {
            view: Default::default(),
            title: title.to_string(),
            left_item: None,
            right_item: None,
            bottom_item: None,
        }
    }
    fn grid_areas(&mut self, schema: &str) -> Self {
        self.view.node_style.push(("grid-template-areas".to_string(), schema.to_string()));
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
            .grid_areas(areas.as_str())
            .view;

        let text = Text::new(self.title.as_str(), TextStyle::LargeTitle)
            .grid_area("title")
            .render();
        view.children.push(text);
        if let Some(left_item) = self.left_item.clone() {
            let mut item = left_item.render();
            item.node_style.push(("grid-area".to_string(), "left_item".to_string()));
            view.children.push(item);
        }
        if let Some(right_item) = self.right_item.clone() {
            let mut item = right_item.render();
            item.node_style.push(("grid-area".to_string(), "right_item".to_string()));
            view.children.push(item);
        }
        if let Some(bottom_item) = self.bottom_item.clone() {
            let mut item = bottom_item.render();
            item.node_style.push(("grid-area".to_string(), "bottom_item".to_string()));
            view.children.push(item);
        }
        view
    }
}