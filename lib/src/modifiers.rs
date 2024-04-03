use crate::node::{NodeContainer, NodeType};
use crate::sp;
use crate::components::{Popover, Popup};
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

pub trait DefaultModifiers<T = Self>: NodeContainer + Clone {
    fn id(&mut self, id: &str) -> Self {
        self.get_node().attributes.insert("id".to_string(), id.to_string());
        self.clone()
    }
    fn color(&mut self, color: &str) -> Self {
        self.get_node().node_style.push(("color".to_string(), color.to_string()));
        self.clone()
    }
    fn opacity(&mut self, opacity: f32) -> Self {
        self.get_node().node_style.push(("opacity".to_string(), opacity.to_string()));
        self.clone()
    }
    fn add_class(&mut self, class_name: &str) -> Self {
        self.get_node().class_list.insert(class_name.to_string());
        self.clone()
    }
    fn remove_class(&mut self, class_name: &str) -> Self {
        self.get_node().class_list.remove(class_name);
        self.clone()
    }
    fn position(&mut self, position: Position) -> Self {

        self.get_node().node_style.push(("position".to_string(), match position {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed"
        }.to_string()));
        self.clone()
    }
    fn padding(&mut self, padding: Vec<i32>) -> Self {
        let params: Vec<String> = padding.iter().map(|size| sp(size.clone())).collect();
        self.get_node().node_style.push(("padding".to_string(), params.join(" ")));
        self.clone()
    }
    fn padding_top(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("padding-top".to_string(), sp(value)));
        self.clone()
    }
    fn padding_bottom(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("padding-bottom".to_string(), sp(value)));
        self.clone()
    }
    fn padding_left(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("padding-left".to_string(), sp(value)));
        self.clone()
    }
    fn padding_right(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("padding-right".to_string(), sp(value)));
        self.clone()
    }
    fn margin(&mut self, margin: Vec<i32>) -> Self {
        let params: Vec<String> = margin.iter().map(|size| sp(size.clone())).collect();
        self.get_node().node_style.push(("margin".to_string(), params.join(" ")));
        self.clone()
    }
    fn margin_top(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("margin-top".to_string(), sp(value)));
        self.clone()
    }
    fn margin_bottom(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("margin-bottom".to_string(), sp(value)));
        self.clone()
    }
    fn margin_left(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("margin-left".to_string(), sp(value)));
        self.clone()
    }
    fn margin_right(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("margin-right".to_string(), sp(value)));
        self.clone()
    }
    fn width(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("width".to_string(), value.to_string()));
        self.clone()
    }
    fn height(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("height".to_string(), value.to_string()));
        self.clone()
    }
    fn min_width(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("min-width".to_string(), value.to_string()));
        self.clone()
    }
    fn min_height(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("min-height".to_string(), value.to_string()));
        self.clone()
    }
    fn max_width(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("max-width".to_string(), value.to_string()));
        self.clone()
    }
    fn max_height(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("max-height".to_string(), value.to_string()));
        self.clone()
    }
    fn sticky_to_top(&mut self, top: i32) -> Self {
        self.get_node().node_style.push(("position".to_string(), "sticky".to_string(), ));
        self.get_node().node_style.push(("top".to_string(), sp(top)));
        self.clone()
    }
    fn sticky_to_bottom(&mut self, bottom: i32) -> Self {
        self.get_node().node_style.push(("position".to_string(), "sticky".to_string(), ));
        self.get_node().node_style.push(("bottom".to_string(), sp(bottom)));
        self.clone()
    }
    fn align_self(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("align-self".to_string(), value.to_string()));
        self.clone()
    }
    fn justify_self(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("justify-self".to_string(), value.to_string()));
        self.clone()
    }
    fn background_color(&mut self, color: &str) -> Self {
        self.get_node().node_style.push(("background-color".to_string(), color.to_string()));
        self.clone()
    }
    fn display(&mut self, display: &str) -> Self {
        self.get_node().node_style.push(("display".to_string(), display.to_string()));
        self.clone()
    }

    fn background_image(&mut self, url: &str) -> Self {
        self.get_node().node_style.push(("background".to_string(), format!("url({}) center / cover", url)));
        self.clone()
    }

    fn background(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("background".to_string(), value.to_string()));
        self.clone()
    }

    fn border(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border".to_string(), value.to_string()));
        self.clone()
    }
    fn border_left(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border-left".to_string(), value.to_string()));
        self.clone()
    }

    fn border_right(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border-right".to_string(), value.to_string()));
        self.clone()
    }

    fn border_bottom(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border-bottom".to_string(), value.to_string()));
        self.clone()
    }

    fn border_top(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border-top".to_string(), value.to_string()));
        self.clone()
    }

    fn tag(&mut self, tag_name: &str) -> Self {
        let self_closing_tags = vec![
            "area",
            "base",
            "br",
            "col",
            "embed",
            "hr",
            "input",
            "img",
            "link",
            "meta",
            "param",
            "source",
            "track",
            "wbr"
        ];
        if self_closing_tags.contains(&tag_name) {
            self.get_node().node_type = NodeType::SelfClosing(tag_name.to_string());
        }
        else {
            self.get_node().node_type = NodeType::Normal(tag_name.to_string());
        }
        self.clone()
    }
    fn set_attr(&mut self, name: &str, value: &str) -> Self {
        self.get_node().attributes.insert(name.to_string(), value.to_string());
        self.clone()
    }

    fn unset_attr(&mut self, name: &str) -> Self {
        self.get_node().attributes.remove(name);
        self.clone()
    }

    fn grid_area(&mut self, name: &str) -> Self {
        self.get_node().node_style.push(("grid-area".to_string(), name.to_string()));
        self.clone()
    }
    fn grid_column(&mut self, column: &str) -> Self {
        self.get_node().node_style.push(("grid-column".to_string(), column.to_string()));
        self.clone()
    }
    fn grid_row(&mut self, row: &str) -> Self {
        self.get_node().node_style.push(("grid-row".to_string(), row.to_string()));
        self.clone()
    }
    fn flex_grow(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("flex-grow".to_string(), value.to_string()));
        self.clone()
    }
    fn line_height(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("line-height".to_string(), value.to_string()));
        self.clone()
    }
    fn box_sizing(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("box-sizing".to_string(), value.to_string()));
        self.clone()
    }
    fn border_radius(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("border-radius".to_string(), value.to_string()));
        self.clone()
    }
    fn text_align(&mut self, value: &str) -> Self {
        self.get_node().node_style.push(("text-align".to_string(), value.to_string()));
        self.clone()
    }
    fn overflow(&mut self, overflow: Overflow) -> Self {
        self.get_node().node_style
            .push(("overflow".to_string(), format!("{:?}", overflow).to_lowercase()));
        self.clone()
    }
    fn aspect_ratio(&mut self, ratio: &str) -> Self {
        self.get_node().node_style
            .push(("object-fit".to_string(), ratio.to_string()));

        self.clone()
    }
    fn popover(&mut self, popover: Popover) -> Self {
        let id = Uuid::new_v4().to_string();
        self.add_class("popover--opener");
        self.set_attr("id", id.as_str());
        self.get_node().root_nodes.insert(Box::new(
            popover.clone().attach_to(id.as_str())
        ));
        self.clone()
    }

    fn popup(&mut self, popup: Popup) -> Self {
        let id = Uuid::new_v4().to_string();
        self.add_class("popup--opener");
        self.set_attr("id", id.as_str());
        self.get_node().root_nodes.insert(Box::new(
            popup.clone().attach_to(id.as_str())
        ));
        self.clone()
    }
}
