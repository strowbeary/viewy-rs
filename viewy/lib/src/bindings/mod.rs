//! This module contains all http framework specific implementations on viewy objects.
//!
//! Supported frameworks :
//!
//! | Supported frameworks | Feature tag |
//! | :--------------------|:-----------:|
//! | Rocket-rs            | `rocket`    |

#[cfg(feature = "rocket")]
pub mod rocket;
