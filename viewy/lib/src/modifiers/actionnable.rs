use crate::bindings::uri::Uri;
use crate::core::component::InteractiveComponentMessage;
use crate::prelude::SheetEdge;
use crate::{core::widget::Widget, node::NodeType};
use short_uuid::short;

/// Describe the different actions that will be triggered
pub enum Action<'a> {
    Navigate {
        url: Uri,
    },
    OpenPopup {
        popup_content_url: Uri,
        display_window_controls: bool, //Idée pour plus tard
    },
    /// Close parent popup or popover
    CloseParentWindow,
    OpenPopover {
        popover_content_url: Uri,
    },
    OpenSheet {
        sheet_content_url: Uri,
        edge: SheetEdge,
        with_backdrop: bool,
    },
    SubmitForm {
        form_name: &'a str,
        inject_into: Option<&'a str>,
    },
    TriggerMessagePayload {
        encoded_message: String,
    },
}

impl Action<'_> {
    /// Build an interactive action that triggers a component message.
    ///
    /// The message is encoded for HTML transport and stored in
    /// `data-v-component-msg`.
    pub fn trigger_message<M>(message: &M) -> Result<Action<'static>, String>
    where
        M: InteractiveComponentMessage,
    {
        Ok(Action::TriggerMessagePayload {
            encoded_message: message.encode_for_transport()?,
        })
    }

    /// Ergonomic constructor for interactive messages.
    ///
    /// This enables API usage such as:
    /// `Action::TriggerMessage(MyMessage::Increment)`.
    #[allow(non_snake_case)]
    pub fn TriggerMessage<M>(message: M) -> Action<'static>
    where
        M: InteractiveComponentMessage,
    {
        Action::trigger_message(&message)
            .unwrap_or_else(|err| panic!("Cannot encode interactive component message: {err}"))
    }

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
            Action::OpenPopup {
                popup_content_url,
                display_window_controls,
            } => {
                let popup_name = short!();
                widget
                    .attributes
                    .insert(format!("data-v-on-{event}"), "open_popup".to_string());
                widget.attributes.insert(
                    "data-v-target-popup".to_string(),
                    format!("popup_{}", popup_name),
                );

                widget.attributes.insert(
                    "data-v-display-window-controls".to_string(),
                    display_window_controls.to_string(),
                );

                widget
                    .attributes
                    .insert("data-v-url".to_string(), popup_content_url.to_string());
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
            Action::OpenPopover {
                popover_content_url,
            } => {
                let popover_name = short!();
                widget
                    .attributes
                    .insert(format!("data-v-on-{event}"), "open_popover".to_string());
                widget.attributes.insert(
                    "data-v-target-popover".to_string(),
                    format!("popover_{}", popover_name),
                );

                widget
                    .attributes
                    .insert("data-v-url".to_string(), popover_content_url.to_string());
            }
            Action::OpenSheet {
                edge,
                sheet_content_url,
                with_backdrop,
            } => {
                widget
                    .attributes
                    .insert(format!("data-v-on-{event}"), "open_sheet".to_string());
                widget
                    .attributes
                    .insert("data-v-sheet-edge".to_string(), edge.to_string());
                if *with_backdrop {
                    widget
                        .attributes
                        .insert("data-v-sheet-with-backdrop".to_string(), "true".to_string());
                }

                widget
                    .attributes
                    .insert("data-v-url".to_string(), sheet_content_url.to_string());
            }
            Action::SubmitForm { form_name, .. } => {}
            Action::TriggerMessagePayload { encoded_message } => {
                widget.attributes.insert(
                    "data-v-component-msg".to_string(),
                    encoded_message.to_string(),
                );
                if event != "click" {
                    widget
                        .attributes
                        .insert("data-v-component-event".to_string(), event.to_string());
                }
            }
            Action::CloseParentWindow => {
                widget.attributes.insert(
                    format!("data-v-on-{event}"),
                    "close_parent_window".to_string(),
                );
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

    fn on_click_message<M>(&mut self, message: &M) -> Result<&mut Self, String>
    where
        M: InteractiveComponentMessage,
    {
        let action = Action::trigger_message(message)?;
        Ok(self.on_click(action))
    }
}

pub trait KeyboardActionnable: Widget {
    fn on_keypress(&mut self, action: Action) -> &mut Self {
        action.apply("keypress", self);
        self
    }
}

pub trait InputActionnable: Widget {
    fn on_change(&mut self, action: Action) -> &mut Self {
        action.apply("change", self);
        self
    }

    fn on_focus(&mut self, action: Action) -> &mut Self {
        action.apply("focus", self);
        self
    }

    fn on_change_message<M>(&mut self, message: &M) -> Result<&mut Self, String>
    where
        M: InteractiveComponentMessage,
    {
        let action = Action::trigger_message(message)?;
        Ok(self.on_change(action))
    }
}
