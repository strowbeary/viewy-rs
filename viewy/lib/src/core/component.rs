use crate::core::node::Node;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Define a struct as a `Component` to use it in `append_child` method of widgets
/// ```rust
///
/// fn sub_component(data: &Homepage) -> View {
///     View::new()
/// }
///
/// use viewy::prelude::*;
/// #[derive(Component)]
/// struct Homepage {
///     pub user_name: String,
/// }
///
/// impl Component for Homepage {
///     fn render(self) -> Node {
///         sub_component(&self).into()
///     }
/// }
pub trait Component: Into<Node> {
    fn name() -> &'static str;

    /// You can write the template of your component with viewy basic widgets or other widgets
    fn render(self) -> Node;
}

/// Stateless interactive component contract.
///
/// # Principle
/// The component state is carried in HTML (hidden field) and sent back to the
/// server on each interaction. The server applies one message and returns the
/// updated component HTML.
///
/// This trait is intentionally small for DX:
/// - `on_message` mutates state from one typed message
/// - `render` returns the component subtree
///
/// Registration and routing are handled by:
/// - `#[derive(InteractiveComponent)]`
/// - `crate::bindings::rocket::component::handle_registered_component_event`
///
/// # Example
/// ```rust
/// use rocket::serde::{Deserialize, Serialize};
/// use viewy::prelude::*;
///
/// #[derive(Serialize, Deserialize, InteractiveComponentMessage)]
/// #[serde(crate = "rocket::serde")]
/// enum CounterMessage {
///     Increment,
///     Decrement,
/// }
///
/// #[derive(Serialize, Deserialize, InteractiveComponent)]
/// #[serde(crate = "rocket::serde")]
/// #[component(messages = CounterMessage)]
/// struct CounterComponent {
///     value: i32,
/// }
///
/// impl InteractiveComponent for CounterComponent {
///     type Message = CounterMessage;
///
///     fn on_message(mut self, message: Self::Message) -> Self {
///         match message {
///             CounterMessage::Increment => self.value += 1,
///             CounterMessage::Decrement => self.value -= 1,
///         }
///         self
///     }
///
///     fn render(self) -> Node {
///         let mut root = View::new();
///         let mut inc = Button::new("+1", ButtonStyle::Filled);
///         inc.on_click(Action::TriggerMessage(CounterMessage::Increment));
///         root.append_child(inc);
///         root.into()
///     }
/// }
/// ```
pub trait InteractiveComponent: Serialize + DeserializeOwned + Sized {
    type Message: DeserializeOwned;
    fn on_message(self, message: Self::Message) -> Self;
    fn render(self) -> Node;

    /// Build a Rocket interactive host for this component.
    ///
    /// The host includes runtime attributes and embeds `_v_component_state`.
    /// It targets the unique interactive route:
    /// `/interactive-components/event`.
    #[cfg(feature = "rocket")]
    fn into_interactive_host(
        self,
        component_id: &str,
    ) -> Result<crate::widgets::view::View, String> {
        crate::bindings::rocket::component::interactive_component_host_with_id(component_id, self)
    }

    /// Build a Rocket interactive host with an auto-generated component id.
    #[cfg(feature = "rocket")]
    fn into_interactive_host_auto(self) -> Result<crate::widgets::view::View, String> {
        crate::bindings::rocket::component::interactive_component_host(self)
    }
}

#[cfg(feature = "rocket")]
impl<T> From<T> for Node
where
    T: InteractiveComponent,
{
    fn from(value: T) -> Self {
        value
            .into_interactive_host_auto()
            .unwrap_or_else(|err| panic!("Cannot build interactive component host: {err}"))
            .into()
    }
}

/// Marker trait for interactive component messages.
///
/// Derive it with `#[derive(InteractiveComponentMessage)]` to get:
/// - transport encoding (`hex:<json>`)
/// - transport decoding from form payload values
pub trait InteractiveComponentMessage: Serialize + DeserializeOwned + Sized {
    /// Encode this message for HTML transport.
    fn encode_for_transport(&self) -> Result<String, String> {
        let json = serde_json::to_string(self)
            .map_err(|err| format!("Cannot serialize component message: {err}"))?;
        Ok(format!("hex:{}", hex::encode(json)))
    }

    /// Decode one message from an encoded transport payload.
    fn decode_from_transport(raw_value: &str) -> Result<Self, String> {
        let json = decode_transport_payload(raw_value)?;
        serde_json::from_str(&json)
            .map_err(|err| format!("Cannot deserialize component message: {err}"))
    }
}

fn decode_transport_payload(raw_value: &str) -> Result<String, String> {
    if let Some(raw_hex) = raw_value.strip_prefix("hex:") {
        let bytes = hex::decode(raw_hex).map_err(|err| format!("Invalid hex payload: {err}"))?;
        return String::from_utf8(bytes).map_err(|err| format!("Invalid UTF-8 payload: {err}"));
    }

    if let Some(raw_json) = raw_value.strip_prefix("json:") {
        return Ok(raw_json.to_string());
    }

    Ok(raw_value.to_string())
}
