use crate::{DefaultModifiers, Renderable};
use crate::components::View;
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct AppLayout {
    desktop_navigation_view: Option<Box<dyn Renderable>>,
    mobile_navigation_view: Option<Box<dyn Renderable>>,
    main_content: Option<Box<dyn Renderable>>,
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
            T: 'static + Renderable, {
        self.desktop_navigation_view = Some(Box::new(item));
        self.clone()
    }

    pub fn mobile_navigation_view<T>(&mut self, item: T) -> Self
        where
            T: 'static + Renderable, {
        self.mobile_navigation_view = Some(Box::new(item));
        self.clone()
    }


    pub fn main_content<T>(&mut self, item: T) -> Self
        where
            T: 'static + Renderable {
        self.main_content = Some(Box::new(item));
        self.clone()
    }
}

impl Renderable for AppLayout {
    fn render(self) -> Node {
        let mut view = View::new();
        view.add_class("app-layout");


        if let Some(desktop_navigation_view) = self.desktop_navigation_view {
            let mut top_item_node = desktop_navigation_view.render();
            top_item_node.class_list.insert("app-layout__navigation-view".to_string());
            top_item_node.class_list.insert("app-layout__navigation-view--desktop".to_string());
            view.get_node().children.push(top_item_node);
        }

        if let Some(top_item) = self.mobile_navigation_view {
            let mut top_item_node = top_item.render();
            top_item_node.class_list.insert("app-layout__navigation-view".to_string());
            top_item_node.class_list.insert("app-layout__navigation-view--mobile".to_string());
            view.get_node().children.push(top_item_node);
        }

        if let Some(main_content) = self.main_content {
            let mut main_content_node = main_content.render();
            main_content_node.class_list.insert("app-layout__main-content".to_string());
            view.get_node().children.push(main_content_node);
        }
        view.render()
    }
}