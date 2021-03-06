use crate::node::{NodeContainer, NodeType};
use crate::sp;
use crate::components::Popover;
use uuid::Uuid;

pub trait DefaultModifiers<T = Self>: NodeContainer + Clone {
    fn color(&mut self, color: &str) -> Self {
        self.get_node().node_style.push(("color".to_string(), color.to_string()));
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
    fn sticky(&mut self, top: i32) -> Self {
        self.get_node().node_style.push(("position".to_string(), "sticky".to_string(), ));
        self.get_node().node_style.push(("top".to_string(), sp(top)));
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

    fn background_image(&mut self, url: &str) -> Self {
        self.get_node().node_style.push(("background".to_string(), format!("url({}) center / cover", url)));
        self.clone()
    }
    fn tag(&mut self, tag_name: &str) -> Self {
        self.get_node().node_type = NodeType::Normal(tag_name.to_string());
        self.clone()
    }
    fn set_attr(&mut self, name: &str, value: &str) -> Self {
        self.get_node().attributes.push((name.to_string(), value.to_string()));
        self.clone()
    }
    fn grid_area(&mut self, name: &str) -> Self {
        self.get_node().node_style.push(("grid-area".to_string(), name.to_string()));
        self.clone()
    }
    fn grid_column(&mut self, column: i32) -> Self {
        self.get_node().node_style.push(("grid-column".to_string(), column.to_string()));
        self.clone()
    }
    fn grid_row(&mut self, row: i32) -> Self {
        self.get_node().node_style.push(("grid-row".to_string(), row.to_string()));
        self.clone()
    }
    fn flex_grow(&mut self, value: i32) -> Self {
        self.get_node().node_style.push(("flex-grow".to_string(), value.to_string()));
        self.clone()
    }
    fn popover(&mut self, popover: Popover) -> Self {
        let id = Uuid::new_v4().to_hyphenated().to_string();
        self.set_attr("id", id.as_str());
        self.get_node().popover = Box::new(Some(
            popover.clone().attach_to(id.as_str())
        ));
        self.clone()
    }
}
