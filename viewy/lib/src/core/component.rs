//! # Interactive Components
//!
//! Interactive components in Viewy follow a hypermedia model:
//! - component state is serialized into HTML (`data-v-component-state`)
//! - one typed message is sent per user interaction
//! - the server applies `on_message` and returns a fresh HTML fragment
//!
//! ## Why this model?
//!
//! - **Simple backend contract**: one route, one typed message, one render.
//! - **No server-side session state required**: state is carried by the DOM.
//! - **Predictable behavior**: each interaction is a pure state transition.
//!
//! ## Quick Start (Concrete Component)
//!
//! ```rust
//! use rocket::serde::{Deserialize, Serialize};
//! use viewy::prelude::*;
//!
//! #[derive(Serialize, Deserialize, InteractiveComponentMessage)]
//! #[serde(crate = "rocket::serde")]
//! enum CounterMessage {
//!     Increment,
//!     Decrement,
//! }
//!
//! #[derive(Serialize, Deserialize, InteractiveComponent)]
//! #[serde(crate = "rocket::serde")]
//! struct CounterComponent {
//!     value: i32,
//! }
//!
//! impl InteractiveComponent for CounterComponent {
//!     type Message = CounterMessage;
//!
//!     fn on_message(mut self, message: Self::Message) -> Self {
//!         match message {
//!             CounterMessage::Increment => self.value += 1,
//!             CounterMessage::Decrement => self.value -= 1,
//!         }
//!         self
//!     }
//!
//!     fn render(self) -> Node {
//!         let mut root = View::new();
//!         let mut inc = Button::new("+1", ButtonStyle::Filled);
//!         inc.on_click(Action::TriggerMessage(CounterMessage::Increment));
//!         root.append_child(inc);
//!         root.into()
//!     }
//! }
//! ```
//!
//! ## Generic Components
//!
//! For generic component definitions (`MyComponent<T>`), register concrete
//! instantiations at compile time:
//!
//! ```rust,ignore
//! use viewy::register_interactive_component;
//!
//! register_interactive_component!(
//!     MyComponent<Book>,
//!     MyComponent<Movie>,
//! );
//! ```
//!
//! Notes:
//! - `#[derive(InteractiveComponent)]` is intended for concrete types.
//! - `register_interactive_component!(...)` is intended for concrete generic instantiations.
//!
//! ## Common Compile Errors
//!
//! - `#[component(...)]` attribute on interactive components:
//!   remove it and define only `type Message = ...` in `impl InteractiveComponent`.
//! - derive on non-struct item:
//!   move state into a `struct`.
//! - generic derive:
//!   use `register_interactive_component!(Type<Concrete>)`.
//!
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
/// The component state is carried in HTML (`data-v-component-state`) and sent back to the
/// server on each interaction. The server applies one message and returns the
/// updated component HTML.
///
/// This trait is intentionally small for DX:
/// - `on_message` mutates state from one typed message
/// - `render` returns the component subtree
///
/// Registration and routing are handled by:
/// - `#[derive(InteractiveComponent)]`
/// - `register_interactive_component!(Type<Concrete>)` for generic component definitions
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
    /// The host includes runtime attributes and embeds
    /// `data-v-component-state`.
    /// It targets the unique interactive route:
    /// `/interactive-components/event`.
    #[cfg(feature = "rocket")]
    fn into_interactive_host(self, component_id: &str) -> Result<Node, String>
    where
        Self: InteractiveComponentMetadata,
    {
        crate::bindings::rocket::component::interactive_component_root_with_id(component_id, self)
    }

    /// Build a Rocket interactive host with an auto-generated component id.
    #[cfg(feature = "rocket")]
    fn into_interactive_host_auto(self) -> Result<Node, String>
    where
        Self: InteractiveComponentMetadata,
    {
        crate::bindings::rocket::component::interactive_component_root(self)
    }
}

/// Metadata required by the interactive component registry/runtime.
///
/// Implemented by interactive component macros to keep host runtime naming
/// and registry naming aligned.
pub trait InteractiveComponentMetadata {
    const REGISTRATION_NAME: &'static str;
}

#[cfg(feature = "rocket")]
impl<T> From<T> for Node
where
    T: InteractiveComponent + InteractiveComponentMetadata,
{
    fn from(value: T) -> Self {
        value
            .into_interactive_host_auto()
            .unwrap_or_else(|err| panic!("Cannot build interactive component host: {err}"))
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
