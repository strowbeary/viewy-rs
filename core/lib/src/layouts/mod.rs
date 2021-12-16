use crate::components::Menu;
use crate::Renderable;

#[derive(Debug, Clone)]
struct AppLayout {
    menu: Option<Menu>,
    bottom_item: Option<Box<dyn Renderable>>,
    top_item: Option<Box<dyn Renderable>>,
}

impl AppLayout {
    pub fn main_menu(&mut self, menu: Menu) -> Self
    {
        self.menu = Some(menu);
        self.clone()
    }

    pub fn bottom_item<T>(&mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.bottom_item = Some(Box::new(item));
        self.clone()
    }

    pub fn top_item<T>(&mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.top_item = Some(Box::new(item));
        self.clone()
    }
}