//! Implementations specific to Page struct
use crate::core::page::{Page, RenderMode};
use rocket::http::ContentType;
use rocket::http::hyper::header::CACHE_CONTROL;
use rocket::log::private::info;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::{Request, debug};

impl<'r> Responder<'r, 'static> for Page<'_> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let render_mode = match request
            .headers()
            .get_one("x-viewy-render-mode")
            .unwrap_or("Complete")
        {
            "LayoutOnly" => RenderMode::LayoutOnly,
            "ContentOnly" => RenderMode::ContentOnly,
            "Complete" | _ => RenderMode::Complete,
        };
        debug!("Rendering mode: {:?}", render_mode);
        let page_content = self.compile(render_mode);
        let mut response = page_content.respond_to(request)?;
        response.set_header(ContentType::HTML);
        response.set_raw_header("Cache-Control", "max-age=3600, private");
        response.set_raw_header("Vary", "x-viewy-render-mode");
        Ok(response)
    }
}
