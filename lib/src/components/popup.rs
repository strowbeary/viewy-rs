use crate::components::icons::Lucide;
use crate::components::{Appendable, Button, ButtonStyle, View};
use crate::engine::IntoPopup;
use crate::node::Node;
use crate::{DefaultModifiers, Renderable};

#[derive(Debug, Clone)]
pub struct Popup {
    pub node: Node,
    pub el_to_attach_to: String,
    form_to_submit_on_open: Option<String>,
    form_to_submit_on_close: Option<String>,
    pub window_controls: bool,
    pub open: bool,
}

impl DefaultModifiers for Popup {}

impl Popup {
    pub fn new() -> Self {
        Popup {
            node: Default::default(),
            el_to_attach_to: "".to_string(),
            form_to_submit_on_open: None,
            form_to_submit_on_close: None,
            window_controls: true,
            open: false,
        }
    }

    pub fn open(&mut self, is_open: bool) -> &mut Self {
        self.open = is_open;
        self
    }

    pub fn hide_window_controls(&mut self) -> &mut Self {
        self.window_controls = false;
        self
    }

    pub fn on_open_submit_form(&mut self, form_name: &str) -> &mut Self {
        self.form_to_submit_on_open = Some(form_name.to_string());
        self
    }

    pub fn on_close_submit_form(&mut self, form_name: &str) -> &mut Self {
        self.form_to_submit_on_close = Some(form_name.to_string());
        self
    }
}

impl std::ops::Deref for Popup {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Popup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Appendable for Popup {}

impl IntoPopup for Popup {
    fn render(self) -> Node {
        Renderable::render(self)
    }

    fn attach_to(&mut self, el: &str) -> &mut Self {
        self.el_to_attach_to = el.to_string();
        self
    }
}

impl Renderable for Popup {
    fn component_name(&self) -> &str {
        "Popup"
    }
    fn render(mut self) -> Node {
        let mut popup = View::new();
        popup
            .add_class("popup")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str());
        if let Some(form_name) = &self.form_to_submit_on_open {
            popup.set_attr("data-form-to-submit-on-open", form_name);
        }

        if let Some(form_name) = &self.form_to_submit_on_close {
            popup.set_attr("data-form-to-submit-on-close", form_name);
        }
        if self.open {
            popup.add_class("visible");
        } else {
            popup.remove_class("visible");
        }

        let mut popup_window = View::new();
        popup_window.add_class("popup__window");

        if self.window_controls {
            popup_window.node.children.push(
                {
                    let mut view = View::new();
                    view.add_class("popup__window-bar").append_child(
                        Button::icon_only(Lucide::X, ButtonStyle::Link)
                            .add_class("popup__window-controls"),
                    );
                    view
                }
                .render(),
            );
        }
        self.add_class("popup__window-content");

        popup_window.node.children.push(self.node);

        popup.node.children.push(popup_window.render());

        popup.node
    }
}
