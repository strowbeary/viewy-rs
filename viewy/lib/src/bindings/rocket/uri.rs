use rocket::http::uri::Reference;

pub(in crate::bindings) type Uri = Reference<'static>;
