use crate::{core::widget::Widget, node::NodeType};

/// Describe the different actions that will be triggered
pub enum Action {
    Navigate {
        url: &'static str,
    },
    OpenPopup {
        popup_content_url: &'static str,
    },
    OpenPopover {},
    LoadDynamicContent {
        dynamic_content_id: &'static str,
        url: &'static str,
    },
}
impl Action {
    /// Apply necessary modification depending on the action so the javascript can act accordingly
    pub fn apply<T>(&self, event: &str, widget: &mut T)
    where
        T: Widget,
    {
        match self {
            Action::Navigate { url } => {
                widget.node_type = NodeType::Normal("a");
                widget
                    .attributes
                    .insert("href".to_string(), url.to_string());
            }
            Action::OpenPopup { popup_content_url } => {
                widget
                    .attributes
                    .insert(format!("data-v-{event}"), "open_popup".to_string());
                widget
                    .attributes
                    .insert("data-v-url".to_string(), popup_content_url.to_string());
            }
            Action::LoadDynamicContent {
                dynamic_content_id,
                url,
            } => {
                widget.attributes.insert(
                    format!("data-v-{event}"),
                    "load_dynamic_content".to_string(),
                );
                widget.attributes.insert(
                    "data-v-dynamic-content-id".to_string(),
                    dynamic_content_id.to_string(),
                );
                widget
                    .attributes
                    .insert("data-v-url".to_string(), url.to_string());
            }
            Action::OpenPopover {} => todo!(),
        }
    }
}

pub trait OnClickActionnable: Widget {
    fn on_click(&mut self, action: Action) -> &mut Self {
        action.apply("on-click", self);
        self
    }
}

pub trait KeyboardActionnable: Widget {
    fn on_keypress(&mut self, action: Action) -> &mut Self {
        action.apply("on-keypress", self);
        self
    }
}

pub trait InputChangeActionnable: Widget {
    fn on_change(&mut self, action: Action) -> &mut Self {
        action.apply("on-change", self);
        self
    }
}
