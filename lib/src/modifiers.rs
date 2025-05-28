use std::ops::{Deref, DerefMut};

use crate::components::{Popover, Popup};
use crate::node::{Node, NodeType};
use crate::{Renderable, sp};
use uuid::Uuid;

#[derive(Debug)]
pub enum Overflow {
    Auto,
    Hidden,
}

#[derive(Debug)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
}

pub trait DefaultModifiers: Deref<Target = Node> + DerefMut {
    fn id(&mut self, id: &str) -> &mut Self {
        self.deref_mut()
            .attributes
            .insert("id".to_string(), id.to_string());
        self
    }
    fn color(&mut self, color: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("color".to_string(), color.to_string()));
        self
    }
    fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("opacity".to_string(), opacity.to_string()));
        self
    }
    fn add_class(&mut self, class_name: &str) -> &mut Self {
        self.deref_mut().class_list.insert(class_name.to_string());
        self
    }
    fn remove_class(&mut self, class_name: &str) -> &mut Self {
        self.deref_mut().class_list.remove(class_name);
        self
    }
    fn position(&mut self, position: Position) -> &mut Self {
        self.deref_mut().node_style.push((
            "position".to_string(),
            match position {
                Position::Static => "static",
                Position::Relative => "relative",
                Position::Absolute => "absolute",
                Position::Fixed => "fixed",
            }
            .to_string(),
        ));
        self
    }
    fn padding(&mut self, padding: Vec<i32>) -> &mut Self {
        let params: Vec<String> = padding.iter().map(|size| sp(size.clone())).collect();
        self.deref_mut()
            .node_style
            .push(("padding".to_string(), params.join(" ")));
        self
    }
    fn padding_top(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("padding-top".to_string(), sp(value)));
        self
    }
    fn padding_bottom(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("padding-bottom".to_string(), sp(value)));
        self
    }
    fn padding_left(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("padding-left".to_string(), sp(value)));
        self
    }
    fn padding_right(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("padding-right".to_string(), sp(value)));
        self
    }
    fn margin(&mut self, margin: Vec<i32>) -> &mut Self {
        let params: Vec<String> = margin.iter().map(|size| sp(size.clone())).collect();
        self.deref_mut()
            .node_style
            .push(("margin".to_string(), params.join(" ")));
        self
    }
    fn margin_top(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("margin-top".to_string(), sp(value)));
        self
    }
    fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("margin-bottom".to_string(), sp(value)));
        self
    }
    fn margin_left(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("margin-left".to_string(), sp(value)));
        self
    }
    fn margin_right(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("margin-right".to_string(), sp(value)));
        self
    }
    fn width(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("width".to_string(), value.to_string()));
        self
    }
    fn height(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("height".to_string(), value.to_string()));
        self
    }
    fn min_width(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("min-width".to_string(), value.to_string()));
        self
    }
    fn min_height(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("min-height".to_string(), value.to_string()));
        self
    }
    fn max_width(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("max-width".to_string(), value.to_string()));
        self
    }
    fn max_height(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("max-height".to_string(), value.to_string()));
        self
    }
    fn sticky_to_top(&mut self, top: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("position".to_string(), "sticky".to_string()));
        self.deref_mut()
            .node_style
            .push(("top".to_string(), sp(top)));
        self
    }
    fn sticky_to_bottom(&mut self, bottom: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("position".to_string(), "sticky".to_string()));
        self.deref_mut()
            .node_style
            .push(("bottom".to_string(), sp(bottom)));
        self
    }
    fn align_self(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("align-self".to_string(), value.to_string()));
        self
    }
    fn justify_self(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("justify-self".to_string(), value.to_string()));
        self
    }
    fn background_color(&mut self, color: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("background-color".to_string(), color.to_string()));
        self
    }
    fn display(&mut self, display: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("display".to_string(), display.to_string()));
        self
    }

    fn background_image(&mut self, url: &str) -> &mut Self {
        self.deref_mut().node_style.push((
            "background".to_string(),
            format!("url({}) center / cover", url),
        ));
        self
    }

    fn background(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("background".to_string(), value.to_string()));
        self
    }

    fn border(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border".to_string(), value.to_string()));
        self
    }
    fn border_left(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border-left".to_string(), value.to_string()));
        self
    }

    fn border_right(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border-right".to_string(), value.to_string()));
        self
    }

    fn border_bottom(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border-bottom".to_string(), value.to_string()));
        self
    }

    fn border_top(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border-top".to_string(), value.to_string()));
        self
    }

    fn tag(&mut self, tag_name: &str) -> &mut Self {
        let self_closing_tags = vec![
            "area", "base", "br", "col", "embed", "hr", "input", "img", "link", "meta", "param",
            "source", "track", "wbr",
        ];
        if self_closing_tags.contains(&tag_name) {
            self.deref_mut().node_type = NodeType::SelfClosing(tag_name.to_string());
        } else {
            self.deref_mut().node_type = NodeType::Normal(tag_name.to_string());
        }
        self
    }
    fn set_attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.deref_mut()
            .attributes
            .insert(name.to_string(), value.to_string());
        self
    }

    fn unset_attr(&mut self, name: &str) -> &mut Self {
        self.deref_mut().attributes.remove(name);
        self
    }

    fn grid_area(&mut self, name: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("grid-area".to_string(), name.to_string()));
        self
    }
    fn grid_column(&mut self, column: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("grid-column".to_string(), column.to_string()));
        self
    }
    fn grid_row(&mut self, row: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("grid-row".to_string(), row.to_string()));
        self
    }
    fn flex_grow(&mut self, value: i32) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("flex-grow".to_string(), value.to_string()));
        self
    }
    fn line_height(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("line-height".to_string(), value.to_string()));
        self
    }
    fn box_sizing(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("box-sizing".to_string(), value.to_string()));
        self
    }
    fn border_radius(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("border-radius".to_string(), value.to_string()));
        self
    }
    fn text_align(&mut self, value: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("text-align".to_string(), value.to_string()));
        self
    }
    fn overflow(&mut self, overflow: Overflow) -> &mut Self {
        self.deref_mut().node_style.push((
            "overflow".to_string(),
            format!("{:?}", overflow).to_lowercase(),
        ));
        self
    }
    fn aspect_ratio(&mut self, ratio: &str) -> &mut Self {
        self.deref_mut()
            .node_style
            .push(("aspect-ratio".to_string(), ratio.to_string()));

        self
    }
    fn popover(&mut self, mut popover: Popover) -> &mut Self {
        let id = Uuid::new_v4().to_string();
        self.add_class("popover--opener");
        self.set_attr("id", id.as_str());
        popover.attach_to(id.as_str());
        self.deref_mut().root_nodes.push(popover.render());
        self
    }

    fn popup(&mut self, mut popup: Popup) -> &mut Self {
        let id = Uuid::new_v4().to_string();
        self.add_class("popup--opener");
        self.set_attr("id", id.as_str());
        popup.attach_to(id.as_str());
        self.deref_mut().root_nodes.push(popup.render());
        self
    }
}
