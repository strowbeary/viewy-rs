use crate::bindings::uri::Uri;
use crate::core::component::InteractiveComponentMessage;
use crate::prelude::SheetEdge;
use crate::{core::widget::Widget, node::NodeType};
use short_uuid::short;

/// Actions that can be attached to widget events.
///
/// `Action::apply` translates each variant into `data-v-*` attributes
/// consumed by the Viewy JavaScript runtime.
pub enum Action<'a> {
    /// Navigate to a URL.
    ///
    /// Runtime effect:
    /// - switches the widget root node to an `<a>`
    /// - sets `href` to the target URL
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::Navigate {
    ///     url: uri!(books_index()),
    /// };
    /// ```
    Navigate { url: Uri },
    /// Open remote content in a popup window.
    ///
    /// Runtime attributes:
    /// - `data-v-on-<event>=open_popup`
    /// - `data-v-url=<popup_content_url>`
    /// - `data-v-target-popup=<generated-popup-id>`
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::OpenPopup {
    ///     popup_content_url: uri!(popup_content()),
    ///     display_window_controls: true,
    /// };
    /// ```
    OpenPopup {
        popup_content_url: Uri,
        display_window_controls: bool, //Idée pour plus tard
    },
    /// Close parent popup or popover.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::CloseParentWindow;
    /// ```
    CloseParentWindow,
    /// Open remote content in a popover.
    ///
    /// Runtime attributes:
    /// - `data-v-on-<event>=open_popover`
    /// - `data-v-url=<popover_content_url>`
    /// - `data-v-target-popover=<generated-popover-id>`
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::OpenPopover {
    ///     popover_content_url: uri!(popover_content()),
    /// };
    /// ```
    OpenPopover { popover_content_url: Uri },
    /// Open remote content in a sheet.
    ///
    /// Runtime attributes:
    /// - `data-v-on-<event>=open_sheet`
    /// - `data-v-url=<sheet_content_url>`
    /// - `data-v-sheet-edge=<edge>`
    /// - optional `data-v-sheet-with-backdrop=true`
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::OpenSheet {
    ///     sheet_content_url: uri!(sheet_content()),
    ///     edge: SheetEdge::Bottom,
    ///     with_backdrop: true,
    /// };
    /// ```
    OpenSheet {
        sheet_content_url: Uri,
        edge: SheetEdge,
        with_backdrop: bool,
    },
    /// Submit a form by name.
    ///
    /// Note: the current `apply` implementation keeps this variant as a
    /// reserved placeholder and does not inject attributes yet.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let action = Action::SubmitForm {
    ///     form_name: "book-form",
    ///     inject_into: None,
    /// };
    /// ```
    SubmitForm {
        form_name: &'a str,
        inject_into: Option<&'a str>,
    },
    /// Trigger one interactive component message.
    ///
    /// Runtime attributes:
    /// - `data-v-component-msg=<encoded-message>`
    /// - optional `data-v-component-event=<event>` for non-click events
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// #[derive(serde::Serialize, serde::Deserialize, InteractiveComponentMessage)]
    /// enum Msg { Increment }
    ///
    /// let action = Action::TriggerMessage(Msg::Increment);
    /// ```
    #[doc(hidden)]
    ComponentMessage { encoded_message: String },
}

impl Action<'_> {
    /// Build an action that triggers one interactive component message.
    ///
    /// The message must be JSON-serializable. Serialization failures are
    /// programmer errors for UI messages, so this constructor panics with a
    /// concrete explanation instead of pushing `Result` through every widget
    /// action helper.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// #[derive(serde::Serialize, serde::Deserialize, InteractiveComponentMessage)]
    /// enum Msg { Increment }
    ///
    /// let action = Action::TriggerMessage(Msg::Increment);
    /// ```
    #[allow(non_snake_case)]
    pub fn TriggerMessage<M>(message: M) -> Action<'static>
    where
        M: InteractiveComponentMessage,
    {
        let encoded_message = message.encode_for_transport().unwrap_or_else(|err| {
            panic!(
                "Cannot encode interactive component message. \
InteractiveComponentMessage types must be JSON-serializable. Error: {err}"
            )
        });

        Action::ComponentMessage { encoded_message }
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
            Action::SubmitForm { .. } => {}
            Action::ComponentMessage { encoded_message } => {
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
    /// Attach an action to the `click` event.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Close", ButtonStyle::Filled);
    /// button.on_click(Action::CloseParentWindow);
    /// ```
    fn on_click(&mut self, action: Action) -> &mut Self {
        action.apply("click", self);
        self
    }

    /// Attach an action to the `dblclick` event.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Open", ButtonStyle::Filled);
    /// button.on_dblclick(Action::CloseParentWindow);
    /// ```
    fn on_dblclick(&mut self, action: Action) -> &mut Self {
        action.apply("dblclick", self);
        self
    }
}

/// Adds keyboard-triggered action helpers.
pub trait KeyboardActionnable: Widget {
    /// Attach an action to the `keypress` event.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut input = Select::new("country", "");
    /// input.on_keypress(Action::CloseParentWindow);
    /// ```
    fn on_keypress(&mut self, action: Action) -> &mut Self {
        action.apply("keypress", self);
        self
    }
}

/// Adds input/focus-triggered action helpers.
pub trait InputActionnable: Widget {
    /// Attach an action to the `change` event.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut input = Select::new("country", "");
    /// input.on_change(Action::CloseParentWindow);
    /// ```
    fn on_change(&mut self, action: Action) -> &mut Self {
        action.apply("change", self);
        self
    }

    /// Attach an action to the `focus` event.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut input = Select::new("country", "");
    /// input.on_focus(Action::CloseParentWindow);
    /// ```
    fn on_focus(&mut self, action: Action) -> &mut Self {
        action.apply("focus", self);
        self
    }
}
