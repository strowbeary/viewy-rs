use crate::core::widget::Widget;
use crate::node::{Node, NodeType};

mod nav_item;
pub use nav_item::NavItem;

#[derive(Widget)]
#[widget(style = "./nav.scss")]
pub struct Nav {
    node: Node,
    level: NavLevel,
    orientation: NavOrientation,
    nav_items: Vec<NavItem>,
}

impl Nav {
    pub fn new(orientation: NavOrientation) -> Self {
        Self {
            node: Node {
                identifier: uuid::Uuid::new_v4(),
                node_type: crate::node::NodeType::Normal("nav"),
                ..Default::default()
            },
            level: NavLevel::Primary,
            orientation,
            nav_items: Vec::new(),
        }
    }

    pub fn level(&mut self, level: NavLevel) -> &mut Self {
        self.level = level;
        self
    }

    pub fn add_item(&mut self, item: NavItem) -> &mut Self {
        self.nav_items.push(item);
        self
    }

    fn render(&mut self) {
        self.node.class_list.insert("nav".to_string());

        match &self.orientation {
            NavOrientation::Horizontal => {
                self.node
                    .class_list
                    .insert("nav-orientation--horizontal".to_string());
            }
            NavOrientation::Vertical => {
                self.node
                    .class_list
                    .insert("nav-orientation--vertical".to_string());
            }
        }

        match &self.level {
            NavLevel::Primary => {
                self.node
                    .class_list
                    .insert("nav-level--primary".to_string());
            }
            NavLevel::Secondary => {
                self.node
                    .class_list
                    .insert("nav-level--secondary".to_string());
            }
            NavLevel::Tertiary => {
                self.node
                    .class_list
                    .insert("nav-level--tertiary".to_string());
            }
        }
        let mut list = Node {
            node_type: NodeType::Normal("ul"),
            ..Default::default()
        };
        for item in &mut self.nav_items {
            list.children.push(item.into());
        }

        self.node.children.push(list);
    }
}

pub enum NavLevel {
    Primary,
    Secondary,
    Tertiary,
}

pub enum NavOrientation {
    Horizontal,
    Vertical,
}
