//! This module contains all http framework specific implementations on viewy objects.
//!
//! Supported frameworks :
//!
//! | Supported frameworks | Feature tag |
//! |:---------------------|:-----------:|
//! | Rocket-rs            | `rocket`    |
//! | axum                 | `axum`      |
//!

#[cfg(feature = "rocket")]
pub mod rocket;

#[cfg(feature = "axum")]
pub mod axum;

pub mod uri {
    #[cfg(feature = "rocket")]
    pub type Uri = crate::bindings::rocket::uri::Uri;
    #[cfg(feature = "axum")]
    pub type Uri = crate::bindings::axum::uri::Uri;
}

#[cfg(any(all(feature = "rocket", feature = "axum"),))]
compile_error!("Features `rocket` and `axum` are mutually exclusive.");
