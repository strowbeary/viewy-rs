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

pub enum UploadResponse {
    Success(&'static str),
    Error(&'static str),
}
impl<'r> Responder<'r, 'static> for UploadResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = Response::build();

        match self {
            UploadResponse::Success(message) => {
                response.status(Status::Ok);
                response.header(Header::new("X-Viewy-Error-Message", message));
            }
            UploadResponse::Error(message) => {
                response.status(Status::InternalServerError);
                response.header(Header::new("X-Viewy-Error-Message", message));
            }
        }
        response.ok()
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
