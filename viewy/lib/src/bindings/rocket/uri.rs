use rocket::http::uri::Origin;

/// Rocket URI type used by Viewy action/navigation helpers.
///
/// Intended usage is with Rocket's `uri!()` macro so routes are checked at
/// compile time.
pub(in crate::bindings) type Uri = Origin<'static>;
