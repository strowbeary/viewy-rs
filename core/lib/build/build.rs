extern crate heck;
extern crate quote;
extern crate scraper;

mod icon_packs;

use crate::icon_packs::generate_icon_pack;


fn main() {
    println!("cargo:rerun-if-changed={}", "build/build.rs");
    generate_icon_pack("Lucide", "lucide/icons");
    generate_icon_pack("SimpleIcons", "simple-icons/icons");

}
