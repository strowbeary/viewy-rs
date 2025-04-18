use rocket::http::Method;
use rocket::route::Outcome;
use rocket::Route;
use crate::Component;
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket::route::Handler;
use crate::router::{Page, RenderMode};
use rocket::data::Data;

impl<'r> Responder<'r, 'static> for Page<'_> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let page_content = self.compile(RenderMode::Complete);
        let mut response = page_content.respond_to(request)?;
        response.set_header(ContentType::HTML);
        Ok(response)
    }
}

