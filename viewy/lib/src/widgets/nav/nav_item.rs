use crate::bindings::uri::Uri;
use crate::core::widget::Widget;
use crate::modifiers::{Action, Appendable};
use crate::node::{Node, NodeType};
use crate::widgets::icon::{Icon, IconPack};
use crate::widgets::text::{Text, TextStyle};
use crate::widgets::view::View;

#[derive(Widget, Clone)]
#[widget(style = "./nav-item.scss")]
pub struct NavItem {
    node: Node,
    label: String,
    url: Uri,
    is_active: bool,
    is_disabled: bool,
    pub icon: Option<Box<dyn IconPack>>,
}

impl NavItem {
    pub fn new(label: &str, url: Uri) -> Self {
        let mut nav_item = Self {
            node: Node {
                identifier: uuid::Uuid::new_v4(),
                node_type: NodeType::Normal("li"),
                ..Default::default()
            },
            label: label.to_string(),
            url,
            is_active: false,
            is_disabled: false,
            icon: None,
        };

        nav_item
    }

    /// Set button's icon
    pub fn icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: 'static + IconPack,
    {
        self.icon = Some(Box::new(icon));
        self
    }

    fn render(&mut self) {
        self.node.class_list.insert("nav-item".to_string());

        let mut link = View::new();

        let action: Action = Action::Navigate {
            url: self.url.clone(),
        };
        action.apply("click", &mut link);

        if let Some(icon_from_pack) = self.icon.clone() {
            let mut icon = Icon::new(icon_from_pack);
            icon.size(16);
            icon.stroke_width(2);
            link.append_child(icon);
        }

        link.append_child(Text::new(&self.label, TextStyle::Body));

        self.node.children.push(link.into());
    }
}

impl Into<NavItem> for &mut NavItem {
    fn into(self) -> NavItem {
        self.clone()
    }
}
