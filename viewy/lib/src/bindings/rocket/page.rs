//! Implementations specific to Page struct
use crate::router::{Page, RenderMode};
use rocket::Request;
use rocket::http::ContentType;
use rocket::response::Responder;

impl<'r> Responder<'r, 'static> for Page<'_> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let page_content = self.compile(RenderMode::Complete);
        let mut response = page_content.respond_to(request)?;
        response.set_header(ContentType::HTML);
        Ok(response)
    }
}
