#[macro_use]
extern crate rocket;
extern crate viewy;

use rocket::response::content::{Css, Html, JavaScript};
use rocket::State;

use viewy::*;
use viewy::components::*;
use crate::catchers::routes;
use viewy::engine::Assets;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

mod catchers;
mod components;
mod layouts;
mod pages;

#[get("/app.css")]
fn get_stylesheet(assets: &State<Assets>) -> Css<String> {
	Css(assets.inner().clone().stylesheet)
}

#[get("/app.js")]
fn get_scripts(assets: &State<Assets>) -> JavaScript<String> {
	JavaScript(assets.inner().clone().script)
}

#[get("/")]
fn home() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Home",
			&layouts::default_layout,
			pages::home()
				.append_child({
					Snackbar::new("Une erreur est survenue")
						.closable()
				}),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/login")]
fn login() -> Html<String> {
	Html({
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
fn table() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Table",
			&layouts::default_layout,
			pages::table(),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/calendar")]
fn calendar() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Calendar",
			&layouts::default_layout,
			pages::calendar(),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/menus")]
fn menus() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Navigation & menus",
			&layouts::default_layout,
			pages::navigation_and_menus(),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/tabs")]
fn tabs() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Tab view",
			&layouts::default_layout,
			pages::tabs(),
		)
			.compile(RenderMode::Complete)
	})
}


#[get("/search")]
fn search_page() -> Html<String> {
	Html({
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
) -> Html<String> {
	Html({
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
fn signature() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Signature field",
			&layouts::default_layout,
			pages::signature_field(),
		)
			.compile(RenderMode::Complete)
	})
}

#[get("/forms")]
fn forms() -> Html<String> {
	Html({
		Page::new(
			"Viewy showcase – Forms",
			&layouts::default_layout,
			pages::forms(),
		)
			.compile(RenderMode::Complete)
	})
}

#[launch]
fn rocket() -> _ {
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
            tabs
        ])
		.register("/", catchers::routes())
		.manage(Assets::new())
}
