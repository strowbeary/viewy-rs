use crate::{View, StyleRegistery};
use std::sync::Mutex;
use crate::component_style;
use crate::template_compilation_tools::ScriptRegistry;

#[derive(Clone, Debug)]
pub enum HorizontalAlignment {
    Center,
    Leading,
    Trailing,
}

pub struct VStack {
    pub alignment: HorizontalAlignment,
    pub gap: (u32, u32),
    pub class_list: Vec<String>,
    pub children: Vec<Box<dyn View>>,
}

impl VStack {
    pub fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + View,
    {
        self.children.push(Box::new(child));
    }
}

impl View for VStack {
    fn get_html(&self) -> String {
        let classes = self.class_list.join(" ");
        let content: Vec<String> = self.children.iter()
            .map(|child| child.get_html())
            .collect();
        let alignment_class = format!("v-stack--align-{:?}", self.alignment).to_lowercase();
        format!(
            "<div class=\"{} {}\">{}</div>",
            classes,
            alignment_class,
            content.join("")
        )
    }

    fn register_css(&self, style_registery: &mut StyleRegistery) {
        self.children.iter()
            .for_each(|child| child.register_css(style_registery));
        style_registery.register_stylesheet("stack", component_style!("stack"));
    }

    fn register_js(&self, script_registery: &mut ScriptRegistry) {
        self.children.iter()
            .for_each(|child| child.register_js(script_registery));
    }
}

impl Default for VStack {
    fn default() -> Self {
        VStack {
            alignment: HorizontalAlignment::Center,
            gap: (0, 0),
            class_list: vec!["v-stack".to_string()],
            children: vec![],
        }
    }
}