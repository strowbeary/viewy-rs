use crate::node::{NodeContainer, NodeType};
use crate::sp;
use crate::components::{Popover, Popup};
use uuid::Uuid;

#[derive(Debug)]
pub enum Overflow {
    Auto,
    Hidden,
}

pub trait DefaultModifiers<T = Self>: NodeContainer + Clone {
    fn color(&mut self, color: &str) -> &mut Self {
        self.get_node().node_style.push(("color".to_string(), color.to_string()));
        self
    }
    fn add_class(&mut self, class_name: &str) -> &mut Self {
        self.get_node().class_list.insert(class_name.to_string());
        self
    }
    fn remove_class(&mut self, class_name: &str) -> &mut Self {
        self.get_node().class_list.remove(class_name);
        self
    }
    fn padding(&mut self, padding: Vec<i32>) -> &mut Self {
        let params: Vec<String> = padding.iter().map(|size| sp(size.clone())).collect();
        self.get_node().node_style.push(("padding".to_string(), params.join(" ")));
        self
    }
    fn padding_top(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("padding-top".to_string(), sp(value)));
        self
    }
    fn padding_bottom(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("padding-bottom".to_string(), sp(value)));
        self
    }
    fn padding_left(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("padding-left".to_string(), sp(value)));
        self
    }
    fn padding_right(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("padding-right".to_string(), sp(value)));
        self
    }
    fn margin(&mut self, margin: Vec<i32>) -> &mut Self {
        let params: Vec<String> = margin.iter().map(|size| sp(size.clone())).collect();
        self.get_node().node_style.push(("margin".to_string(), params.join(" ")));
        self
    }
    fn margin_top(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("margin-top".to_string(), sp(value)));
        self
    }
    fn margin_bottom(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("margin-bottom".to_string(), sp(value)));
        self
    }
    fn margin_left(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("margin-left".to_string(), sp(value)));
        self
    }
    fn margin_right(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("margin-right".to_string(), sp(value)));
        self
    }
    fn width(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("width".to_string(), value.to_string()));
        self
    }
    fn height(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("height".to_string(), value.to_string()));
        self
    }
    fn min_width(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("min-width".to_string(), value.to_string()));
        self
    }
    fn min_height(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("min-height".to_string(), value.to_string()));
        self
    }
    fn max_width(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("max-width".to_string(), value.to_string()));
        self
    }
    fn max_height(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("max-height".to_string(), value.to_string()));
        self
    }
    fn sticky(&mut self, top: i32) -> &mut Self {
        self.get_node().node_style.push(("position".to_string(), "sticky".to_string(), ));
        self.get_node().node_style.push(("top".to_string(), sp(top)));
        self
    }
    fn align_self(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("align-self".to_string(), value.to_string()));
        self
    }
    fn justify_self(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("justify-self".to_string(), value.to_string()));
        self
    }
    fn background_color(&mut self, color: &str) -> &mut Self {
        self.get_node().node_style.push(("background-color".to_string(), color.to_string()));
        self
    }
    fn display(&mut self, display: &str) -> &mut Self {
        self.get_node().node_style.push(("display".to_string(), display.to_string()));
        self
    }

    fn background_image(&mut self, url: &str) -> &mut Self {
        self.get_node().node_style.push(("background".to_string(), format!("url({}) center / cover", url)));
        self
    }

    fn background(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("background".to_string(), value.to_string()));
        self
    }

    fn border(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border".to_string(), value.to_string()));
        self
    }
    fn border_left(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border-left".to_string(), value.to_string()));
        self
    }

    fn border_right(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border-right".to_string(), value.to_string()));
        self
    }

    fn border_bottom(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border-bottom".to_string(), value.to_string()));
        self
    }

    fn border_top(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border-top".to_string(), value.to_string()));
        self
    }

    fn tag(&mut self, tag_name: &str) -> &mut Self {
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
        self
    }
    fn set_attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.get_node().attributes.push((name.to_string(), value.to_string()));
        self
    }
    fn grid_area(&mut self, name: &str) -> &mut Self {
        self.get_node().node_style.push(("grid-area".to_string(), name.to_string()));
        self
    }
    fn grid_column(&mut self, column: &str) -> &mut Self {
        self.get_node().node_style.push(("grid-column".to_string(), column.to_string()));
        self
    }
    fn grid_row(&mut self, row: &str) -> &mut Self {
        self.get_node().node_style.push(("grid-row".to_string(), row.to_string()));
        self
    }
    fn flex_grow(&mut self, value: i32) -> &mut Self {
        self.get_node().node_style.push(("flex-grow".to_string(), value.to_string()));
        self
    }
    fn line_height(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("line-height".to_string(), value.to_string()));
        self
    }
    fn border_radius(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("border-radius".to_string(), value.to_string()));
        self
    }
    fn text_align(&mut self, value: &str) -> &mut Self {
        self.get_node().node_style.push(("text-align".to_string(), value.to_string()));
        self
    }
    fn overflow(&mut self, overflow: Overflow) -> &mut Self {
        self.get_node().node_style
            .push(("overflow".to_string(), format!("{:?}", overflow).to_lowercase()));
        self
    }
    fn popover(&mut self, popover: Popover) -> &mut Self {
        let id = Uuid::new_v4().to_hyphenated().to_string();
        self.set_attr("id", id.as_str());
        self.get_node().popover = Box::new(Some(
            popover.clone().attach_to(id.as_str())
        ));
        self
    }

    fn popup(&mut self, popup: Popup) -> &mut Self {
        let id = Uuid::new_v4().to_hyphenated().to_string();
        self.set_attr("id", id.as_str());
        self.get_node().popup = Box::new(Some(
            popup.clone().attach_to(id.as_str())
        ));
        self
    }
}
