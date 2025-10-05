use short_uuid::short;
use crate::{core::widget::Widget, node::NodeType};

/// Describe the different actions that will be triggered
pub enum Action<'a> {
    Navigate {
        url: &'a str,
    },
    OpenPopup {
        popup_content_url: Option<&'a str>,
        // display_window_controls: bool //Idée pour plus tard
    },
    OpenPopover {
        popover_content_url: Option<&'a str>,
    },
    SubmitForm {
        form_name: &'a str,
        inject_into: Option<&'a str>,
    }
}

impl Action<'_> {
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
                let popup_name = short!();
                widget
                    .attributes
                    .insert(format!("data-v-on-{event}"), "open_popup".to_string());
                widget.attributes.insert("data-v-target-popup".to_string(), format!("popup_{}",popup_name));

                if let Some(url) = popup_content_url {
                    widget
                        .attributes
                        .insert("data-v-url".to_string(), url.to_string());
                }

            }
           /* Action::LoadDynamicContent {
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
            }*/
            Action::OpenPopover { popover_content_url } => {
                let popover_name = short!();
                widget
                    .attributes
                    .insert(format!("data-v-on-{event}"), "open_popover".to_string());
                widget.attributes.insert("data-v-target-popover".to_string(), format!("popover_{}",popover_name));

                if let Some(url) = popover_content_url {
                    widget
                        .attributes
                        .insert("data-v-url".to_string(), url.to_string());
                }
            },
            Action::SubmitForm { form_name, .. } => {

            }
        }
    }
}

pub trait OnClickActionnable: Widget {
    fn on_click(&mut self, action: Action) -> &mut Self {
        action.apply("click", self);
        self
    }
    fn on_dblclick(&mut self, action: Action) -> &mut Self {
        action.apply("dblclick", self);
        self
    }
}

pub trait KeyboardActionnable: Widget {
    fn on_keypress(&mut self, action: Action) -> &mut Self {
        action.apply("keypress", self);
        self
    }
}

pub trait InputChangeActionnable: Widget {
    fn on_change(&mut self, action: Action) -> &mut Self {
        action.apply("change", self);
        self
    }
}
