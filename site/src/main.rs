#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{FileServer, TempFile, relative};
use rocket::response::content::{RawCss, RawHtml, RawJavaScript};
use rocket::response::{Responder, Response};
use rocket::{Request, State};
use std::env;
use std::path::Path;

use rocket::form::Form;
use rocket::http::{Header, Status};
use rocket::serde::{Deserialize, Serialize};
use viewy::components::{
    Alignment, Appendable, Card, CardStyle, Popover, Snackbar, SnackbarType, Text, TextStyle,
    VStack,
};
use viewy::engine::Assets;
use viewy::{DefaultModifiers, Page, RenderMode, UploadResponse, scale};

mod catchers;
mod components;
mod layouts;
mod pages;

#[get("/app.css")]
fn get_stylesheet(assets: &State<Assets>) -> RawCss<String> {
    RawCss(assets.inner().clone().stylesheet)
}

#[get("/app.js")]
fn get_scripts(assets: &State<Assets>) -> RawJavaScript<String> {
    RawJavaScript(assets.inner().clone().script)
}

#[get("/")]
fn home() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Home",
            &layouts::default_layout,
            pages::home().append_child({
                Snackbar::new(SnackbarType::Neutral, "Une erreur est survenue").closable()
            }),
        )
        .compile(RenderMode::Complete)
    })
}

#[get("/login")]
fn login() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Login",
            &layouts::login_layout,
            pages::login(),
        )
        .compile(RenderMode::Complete)
    })
}

struct LoginForm {
    username: String,
    password: String,
}

#[get("/table")]
fn table() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Table",
            &layouts::default_layout,
            pages::table(),
        )
        .compile(RenderMode::Complete)
    })
}

#[get("/calendar")]
fn calendar() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Calendar",
            &layouts::default_layout,
            pages::calendar(),
        )
        .compile(RenderMode::Complete)
    })
}

#[get("/menus")]
fn menus() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Navigation & menus",
            &layouts::default_layout,
            pages::navigation_and_menus(),
        )
        .compile(RenderMode::Complete)
    })
}

#[get("/tabs")]
fn tabs() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Tab view",
            &layouts::default_layout,
            pages::tabs(),
        )
        .compile(RenderMode::Complete)
    })
}

#[get("/search")]
fn search_page() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Dynamic content",
            &layouts::default_layout,
            pages::dynamic_content(),
        )
        .compile(RenderMode::Complete)
    })
}

#[derive(FromForm)]
struct SearchForm {
    q: String,
}

#[post("/search", data = "<search_form>")]
fn search_result(search_form: rocket::form::Form<SearchForm>) -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Dynamic content",
            &layouts::default_layout,
            {
                let mut result_stack = VStack::new(Alignment::Stretch).gap(vec![scale(3)]);
                for i in 0..5 {
                    result_stack.append_child({
                        Card::new(CardStyle::Outlined)
                            .padding(vec![scale(4)])
                            .append_child({
                                Text::new(&format!("{} {}", i, search_form.q), TextStyle::H3)
                            })
                            .popover({
                                Popover::new().append_child({
                                    Text::new(&format!("{} {}", i, search_form.q), TextStyle::H3)
                                })
                            })
                    });
                }
                result_stack
            },
        )
        .compile(RenderMode::ContentOnly)
    })
}

#[get("/forms")]
fn forms() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Forms",
            &layouts::default_layout,
            pages::forms(None),
        )
        .compile(RenderMode::Complete)
    })
}
#[post("/forms", data = "<content>")]
fn forms_richtext(content: Form<Option<String>>) -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Forms",
            &layouts::default_layout,
            pages::forms(content.into_inner()),
        )
        .compile(RenderMode::Complete)
    })
}

#[post("/upload-file", data = "<form>")]
async fn upload_file(form: Form<TempFile<'_>>) -> UploadResponse<Status> {
    let mut file = form.into_inner();
    let filename = format!(
        "{}.{}",
        file.name().unwrap_or("hello"),
        file.content_type().and_then(|ct| ct.extension()).unwrap()
    );

    match file.persist_to(Path::new("./uploads").join(filename)).await {
        Ok(_) => UploadResponse::Success(
            "Fichier téléversé avec succès (message serveur)",
            Status::Ok,
        ),
        Err(_) => UploadResponse::Error(
            "Erreur lors de l'upload (message serveur)",
            Status::InternalServerError,
        ),
    }
}

#[get("/table-of-content")]
fn table_of_content() -> RawHtml<String> {
    RawHtml({
        Page::new(
            "Viewy showcase – Table of content",
            &layouts::default_layout,
            pages::table_of_content(),
        )
        .compile(RenderMode::Complete)
    })
}

struct ContentSecurityPolicy;

#[rocket::async_trait]
impl Fairing for ContentSecurityPolicy {
    fn info(&self) -> Info {
        Info {
            name: "Content Security Policy",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
			"Content-Security-Policy",
				"frame-src 'self' data: gap: *.gouvizee.com youtube.com *.youtube.com twitter.com *.twitter.com; script-src 'self' cdn.jsdelivr.net cdn.redoc.ly *.gouvizee.com youtube.com *.youtube.com twitter.com *.twitter.com *.twimg.com unpkg.com 'unsafe-inline'",

		));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(ContentSecurityPolicy)
        .mount(
            "/",
            routes![
                get_stylesheet,
                get_scripts,
                home,
                login,
                table,
                calendar,
                menus,
                search_page,
                search_result,
                forms,
                upload_file,
                tabs,
                table_of_content,
                forms_richtext
            ],
        )
        .mount("/assets", FileServer::from(relative!("assets")))
        .register("/", catchers::routes())
        .manage(Assets::new())
}
