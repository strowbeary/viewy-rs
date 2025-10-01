//! Implementations specific to Page struct
use crate::router::{Page, RenderMode};
use rocket::Request;
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::json;

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

        match render_mode {
            RenderMode::Complete | RenderMode::LayoutOnly => {
                let page_content = self.compile(render_mode);
                let mut response = page_content.respond_to(request)?;
                response.set_header(ContentType::HTML);
                Ok(response)
            }
            RenderMode::ContentOnly => {
                let root_nodes = self.content.get_root_nodes();
                let mut root_nodes_html_buffer = String::new();
                for node in root_nodes {
                    node.render(&mut root_nodes_html_buffer);
                }

                let mut content_html_buffer = String::new();
                self.content.render(&mut content_html_buffer);
                
                let mut response = json!({
                    "content": content_html_buffer,
                    "root_nodes": root_nodes_html_buffer
                }).respond_to(request)?;
                response.set_header(ContentType::JSON);
                Ok(response)
            }
        }


    }
}
