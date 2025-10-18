use rocket::fairing::{AdHoc, Fairing, Info, Kind};
use rocket::fs::{FileServer, Options};
use rocket::http::Header;
use rocket::response::content::RawCss;
use rocket::{Request, Response, Rocket, get, routes};
use std::path::Path;

pub struct CacheFairing;

#[rocket::async_trait]
impl Fairing for CacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add Cache-Control header for static files",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // Vérifie si la requête cible les fichiers statiques (ex: /static/*)
        if req.uri().path().starts_with("/viewy-static/") && res.status().code == 200 {
            res.set_header(Header::new("Cache-Control", "public, max-age=31536000"));
        }
    }
}

#[get("/app.css")]
async fn get_stylesheet() -> RawCss<String> {
    RawCss(crate::prelude::get_stylesheet())
}

pub fn viewy_static_assets_fairing() -> AdHoc {
    println!("Viewy CARGO_MANIFEST_DIR = {}", env!("CARGO_MANIFEST_DIR"));
    AdHoc::on_ignite("Viewy Static Assets", |rocket| async move {
        rocket
            .attach(CacheFairing)
            .mount(
                "/viewy-static/",
                FileServer::from(Path::new(env!("CARGO_MANIFEST_DIR")).join("static")),
            )
            .mount("/viewy-static", routes![get_stylesheet])
    })
}
