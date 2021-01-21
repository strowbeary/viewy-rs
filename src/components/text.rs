use crate::{StyleRegistery, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::view::{View, DefaultModifiers, ViewContainer};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum TextStyle {
    LargeTitle,
    H1,
    H2,
    H3,
    Headline,
    Subtitle1,
    Subtitle2,
    Body1,
    Body2,
    Button,
    Label,
    Overline,
    Caption,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub view: View,
    pub content: String,
    pub style: TextStyle
}
impl ViewContainer for Text {
    fn get_view(&mut self) -> &mut View {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<Text> for Text {}

impl Text {
    pub fn new(content: &str, style: TextStyle) -> Self {
        let mut view = View::default();
        view.text = Some(content.to_string());
        Text {
            view,
            content: content.to_string(),
            style: style
        }
    }

}

impl Renderable for Text {
    fn render(&self, style_registery: &mut StyleRegistery, script_registery: &mut ScriptRegistry) -> View {
        style_registery.register_stylesheet(
            "text",
            include_str!("../themes/components/text.scss"),
        );
        script_registery.register_script(
            "button",
            include_str!("../js/button.js"),
        );
        let text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        text.view
    }
}