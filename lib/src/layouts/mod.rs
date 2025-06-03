use crate::components::View;
use crate::node::Node;
use crate::{DefaultModifiers, Renderable};

#[derive(Debug, Clone)]
pub struct AppLayout {
    pub desktop_navigation_view: Option<Node>,
    pub mobile_navigation_view: Option<Node>,
    pub main_content: Option<Node>,
}

impl AppLayout {
    pub fn new() -> Self {
        Self {
            desktop_navigation_view: None,
            mobile_navigation_view: None,
            main_content: None,
        }
    }

    pub fn desktop_navigation_view<T>(&mut self, item: T) -> Self
    where
        T: 'static + Renderable,
    {
        self.desktop_navigation_view = Some(item.render());
        self.clone()
    }

    pub fn mobile_navigation_view<T>(&mut self, item: T) -> Self
    where
        T: 'static + Renderable,
    {
        self.mobile_navigation_view = Some(item.render());
        self.clone()
    }

    pub fn main_content<T>(&mut self, item: T) -> Self
    where
        T: Renderable,
    {
        self.main_content = Some(item.render());
        self.clone()
    }
}

impl Renderable for AppLayout {
    fn render(self) -> Node {
        let mut view = View::new();
        view.add_class("app-layout");

        if let Some(mut desktop_navigation_view) = self.desktop_navigation_view {
            desktop_navigation_view
                .class_list
                .insert("app-layout__navigation-view".to_string());
            desktop_navigation_view
                .class_list
                .insert("app-layout__navigation-view--desktop".to_string());
            view.node.children.push(desktop_navigation_view);
        }

        if let Some(mut top_item) = self.mobile_navigation_view {
            top_item
                .class_list
                .insert("app-layout__navigation-view".to_string());
            top_item
                .class_list
                .insert("app-layout__navigation-view--mobile".to_string());
            view.node.children.push(top_item);
        }

        if let Some(mut main_content) = self.main_content {
            main_content
                .class_list
                .insert("app-layout__main-content".to_string());
            view.node.children.push(main_content);
        }
        view.render()
    }
}
