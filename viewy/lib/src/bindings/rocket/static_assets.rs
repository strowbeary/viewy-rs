
use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, Options};
use rocket::Rocket;

pub fn viewy_static_assets_fairing() -> AdHoc {
    println!("Viewy CARGO_MANIFEST_DIR = {}", env!("CARGO_MANIFEST_DIR"));
    AdHoc::on_ignite("Viewy Static Assets", |rocket| async move {
        rocket.mount("/viewy-assets/", FileServer::from(format!("{}/assets", env!("CARGO_MANIFEST_DIR")),))
    })
}