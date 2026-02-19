//! Rocket helpers for interactive components handled through a single route.
//!
//! # Model
//! - Component state is transported in HTML (`data-v-component-state` attribute).
//! - User actions send one encoded message (`data-v-component-msg`).
//! - One Rocket endpoint dispatches to the right component implementation.
//!
//! # Required runtime fields
//! The Viewy JS runtime posts form fields with:
//! - `_v_component_name`
//! - `_v_component_id`
//! - `_v_component_msg`
//! - `_v_component_version`
//! - `_v_component_state`
//!
//! Component implementations are registered at compile time via either:
//! - `#[derive(InteractiveComponent)]` for concrete component types
//! - `register_interactive_component!(MyGenericComponent<ConcreteType>)` for generic ones
//!
//! The interactive event endpoint is fixed to:
//! - [`INTERACTIVE_COMPONENT_EVENT_ROUTE`]
//!
//! # Route wiring
//! The route is mounted automatically by `viewy_static_assets_fairing()`.
//! No manual event route is required in your application.

use crate::core::component::{
    InteractiveComponent, InteractiveComponentMessage, InteractiveComponentMetadata,
};
use crate::core::node::Node;
use rocket::State;
use rocket::form::Form;
use rocket::http::Status;
use rocket::post;
use rocket::response::content::RawHtml;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

/// Unique route path expected by the interactive runtime.
pub const INTERACTIVE_COMPONENT_EVENT_ROUTE: &str = "/interactive-components/event";

/// Raw form payload sent by the generic interactive component runtime.
pub type InteractiveComponentEventForm = BTreeMap<String, String>;

/// Registry entry generated at compile time by interactive component macros.
pub struct InteractiveComponentRegistration {
    pub component_name: &'static str,
    pub handler: fn(&InteractiveComponentEventForm) -> Result<Node, String>,
}

impl InteractiveComponentRegistration {
    pub const fn new(
        component_name: &'static str,
        handler: fn(&InteractiveComponentEventForm) -> Result<Node, String>,
    ) -> Self {
        Self {
            component_name,
            handler,
        }
    }
}

::inventory::collect!(InteractiveComponentRegistration);

fn render_node(node: Node) -> String {
    let mut html = String::new();
    node.render(&mut html);
    html
}

/// Dispatch one event to the registered component and return HTML fragment.
///
/// Expected keys:
/// - `_v_component_name`
/// - `_v_component_msg`
/// - `_v_component_state`
///
/// This function is intended to be called by one global Rocket route.
pub async fn handle_registered_component_event(
    mut form: InteractiveComponentEventForm,
) -> Result<String, String> {
    let component_name = form
        .remove("_v_component_name")
        .ok_or_else(|| "Missing `_v_component_name` form field".to_string())?;

    let registration = ::inventory::iter::<InteractiveComponentRegistration>
        .into_iter()
        .find(|entry| entry.component_name == component_name)
        .ok_or_else(|| format!("No interactive component registered as `{component_name}`"))?;

    let next_node = (registration.handler)(&form)?;
    Ok(render_node(next_node))
}

#[post("/interactive-components/event", data = "<payload>")]
pub(crate) async fn interactive_components_event(
    payload: Form<InteractiveComponentEventForm>,
) -> Result<RawHtml<String>, Status> {
    handle_registered_component_event(payload.into_inner())
        .await
        .map(RawHtml)
        .map_err(|_| Status::BadRequest)
}

/// Encode a strongly typed message for safe HTML transport.
///
/// The output uses `hex:<hex-encoded-json>`.
pub fn encode_component_message<M>(message: &M) -> Result<String, String>
where
    M: InteractiveComponentMessage,
{
    message.encode_for_transport()
}

/// Decode a message encoded with [`encode_component_message`].
pub fn decode_component_message<M>(raw_message: &str) -> Result<M, String>
where
    M: InteractiveComponentMessage,
{
    M::decode_from_transport(raw_message)
}

/// Decode a component state encoded in `_v_component_state`.
pub fn decode_component_state<S>(raw_state: &str) -> Result<S, String>
where
    S: DeserializeOwned,
{
    let json = decode_payload(raw_state)?;
    serde_json::from_str(&json).map_err(|err| format!("Cannot deserialize component state: {err}"))
}

/// Build a host node for an interactive component instance with an explicit id/version.
pub fn interactive_component_root_with_id_and_version<C>(
    component_id: &str,
    component_version: u64,
    component: C,
) -> Result<Node, String>
where
    C: InteractiveComponent + InteractiveComponentMetadata,
{
    let serialized_state = encode_component_state(&component)?;
    let mut root = component.render();

    root.attributes
        .insert("data-v-component-host".to_string(), "true".to_string());
    root.attributes.insert(
        "data-v-component-mode".to_string(),
        "hypermedia".to_string(),
    );
    root.attributes.insert(
        "data-v-component-name".to_string(),
        C::REGISTRATION_NAME.to_string(),
    );
    root.attributes
        .insert("data-v-component-id".to_string(), component_id.to_string());
    root.attributes.insert(
        "data-v-component-event-url".to_string(),
        INTERACTIVE_COMPONENT_EVENT_ROUTE.to_string(),
    );
    root.attributes.insert(
        "data-v-component-version".to_string(),
        component_version.to_string(),
    );
    root.attributes
        .insert("data-v-component-state".to_string(), serialized_state);

    Ok(root)
}

/// Build a host node for an interactive component instance with an explicit id.
pub fn interactive_component_root_with_id<C>(
    component_id: &str,
    component: C,
) -> Result<Node, String>
where
    C: InteractiveComponent + InteractiveComponentMetadata,
{
    interactive_component_root_with_id_and_version(component_id, 0, component)
}

/// Build a host node for an interactive component instance.
///
/// A unique `data-v-component-id` is generated automatically.
pub fn interactive_component_root<C>(component: C) -> Result<Node, String>
where
    C: InteractiveComponent + InteractiveComponentMetadata,
{
    let component_id = uuid::Uuid::new_v4().to_string();
    interactive_component_root_with_id(&component_id, component)
}

fn encode_component_state<S>(state: &S) -> Result<String, String>
where
    S: serde::Serialize,
{
    let json = serde_json::to_string(state)
        .map_err(|err| format!("Cannot serialize component state: {err}"))?;
    Ok(format!("hex:{}", hex::encode(json)))
}

fn decode_payload(raw_value: &str) -> Result<String, String> {
    if let Some(raw_hex) = raw_value.strip_prefix("hex:") {
        let bytes = hex::decode(raw_hex).map_err(|err| format!("Invalid hex payload: {err}"))?;
        return String::from_utf8(bytes).map_err(|err| format!("Invalid UTF-8 payload: {err}"));
    }

    if let Some(raw_json) = raw_value.strip_prefix("json:") {
        return Ok(raw_json.to_string());
    }

    Ok(raw_value.to_string())
}
