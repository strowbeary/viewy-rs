#[macro_use]
extern crate rocket;
extern crate viewy;

use std::env;
use std::path::Path;
use rocket::fs::{FileServer, relative, TempFile};
use rocket::response::content::{RawCss, RawHtml, RawJavaScript};
use rocket::State;

use viewy::engine::Assets;
use rocket::serde::{Serialize, Deserialize};
use rocket::form::Form;
use rocket::http::Status;
use viewy::{DefaultModifiers, Page, RenderMode, scale};
use viewy::components::{Alignment, Appendable, Card, CardStyle, Popover, Snackbar, SnackbarType, Text, TextStyle, VStack};

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
			pages::home()
				.append_child({
					Snackbar::new(SnackbarType::Neutral, "Une erreur est survenue")
						.closable()
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
fn search_result(
	search_form: rocket::form::Form<SearchForm>
) -> RawHtml<String> {
	RawHtml({
		Page::new(
			"Viewy showcase – Dynamic content",
			&layouts::default_layout, {
				let mut result_stack = VStack::new(Alignment::Stretch)
					.gap(vec![scale(3)]);
				for i in 0..5 {
					result_stack.append_child({
						Card::new(CardStyle::Outlined)
							.padding(vec![scale(4)])
							.append_child({
								Text::new(&format!("{} {}", i, search_form.q), TextStyle::H3)
							})
							.popover({
							Popover::new()
								.append_child({
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

#[get("/signature")]
fn signature() -> RawHtml<String> {
	RawHtml({
		Page::new(
			"Viewy showcase – Signature field",
			&layouts::default_layout,
			pages::signature_field(),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/forms")]
fn forms() -> RawHtml<String> {
	RawHtml({
		Page::new(
			"Viewy showcase – Forms",
			&layouts::default_layout,
			pages::forms(),
		)
			.compile(RenderMode::Complete)
	})
}



#[post("/upload-file", data = "<form>")]
async fn upload_file(
	form: Form<TempFile<'_>>
) -> Status {
	let mut file = form.into_inner();
	let filename = format!("{}.{}", file.name().unwrap_or("hello"), file.content_type().and_then(|ct| ct.extension()).unwrap());
	match file.persist_to(Path::new("./uploads").join(filename)).await {
		Ok(_) => Status::Ok,
		Err(_) => Status::InternalServerError
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

#[launch]
fn rocket() -> _ {
	println!("{:?}", env::var("VIEWY_COLORS_ACCENT_LIGHT"));
	rocket::build()
		.mount("/", routes![
            get_stylesheet,
            get_scripts,
            home,
            login,
            table,
            calendar,
            menus,
            search_page,
            search_result,
            signature,
            forms,
			upload_file,
            tabs,
			table_of_content
        ])
		.mount("/assets", FileServer::from(relative!("assets")))
		.register("/", catchers::routes())
		.manage(Assets::new())
}
