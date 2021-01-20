use crate::components::View;
use crate::helper_fn::sp;

#[derive(Debug)]
pub enum HorizontalAlignment {
    Center,
    Leading,
    Trailing
}

pub type VStack = View;

impl VStack {
    pub fn init(alignment: HorizontalAlignment) -> Self{
        VStack::default()
            .add_class("stack")
            .add_class("stack--vertical")
            .add_class(format!("stack--vertical--{:?}", alignment).to_lowercase().as_str())
    }
    pub fn gap(&mut self, value: i32) -> Self {
        self.view_style.push(("gap".to_string(), sp(value)));
        self.clone()
    }
}
