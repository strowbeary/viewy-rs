extern crate base64;
extern crate dyn_clone;
extern crate grass;
extern crate minifier;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate uuid;

pub use config::Config;
pub use engine::Renderable;
pub use helper_fn::*;
pub use modifiers::{DefaultModifiers, Overflow, Position};
pub use page::{Page, RenderMode};
use rocket::{
    Request, Response,
    http::{Header, Status},
};

use rocket::response::Responder;

mod helper_fn;
mod modifiers;
mod node;

/// Module containing all ui components of viewy-rs
pub mod components;
pub mod layouts;
mod page;

mod config;
pub mod engine;

pub enum UploadResponse<R> {
    Success(&'static str, R),
    Error(&'static str, R),
}
impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for UploadResponse<R> {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            UploadResponse::Success(message, inner) => Response::build_from(inner.respond_to(req)?)
                .header(Header::new("X-Viewy-Message", urlencoding::encode(message)))
                .ok(),
            UploadResponse::Error(message, inner) => Response::build_from(inner.respond_to(req)?)
                .status(Status::InternalServerError)
                .header(Header::new("X-Viewy-Message", urlencoding::encode(message)))
                .ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Appendable, Button, ButtonStyle, View};
    use crate::{Page, RenderMode, Renderable};
    use std::time::Instant;

    #[test]
    fn benchmark() {
        let start = Instant::now();
        let mut view = View::new();
        for i in 0..50000 {
            view.append_child(Button::new("Label", ButtonStyle::Filled));
        }
        let render_start = Instant::now();
        let _ = Page::new(
            "Render test",
            &|content| Box::new(View::new().append_child(content)),
            view,
        )
        .compile(RenderMode::Complete);

        let render_duration = render_start.elapsed();
        let duration = start.elapsed();
        println!("Render duration = {:?}", render_duration);
        println!("Global duration = {:?}", duration);
    }
}
