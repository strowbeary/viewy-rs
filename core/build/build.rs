extern crate heck;
extern crate quote;
extern crate scraper;

mod icon_packs;

use std::env;
use std::path::Path;
use quote::quote;
use crate::icon_packs::generate_icon_pack;


fn main() {
    println!("cargo:rerun-if-changed={}", "build/*");
    let mut code = quote! {
         use crate::modifiers::DefaultModifiers;
    }
        .to_string();
    code += &generate_icon_pack("Lucide", true, "assets/lucide/icons", None);
    code += &generate_icon_pack("SimpleIcons", false, "assets/simple-icons/icons", Some("Icon"));

    std::fs::write(
        Path::new(&env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable"))
            .join("icons.rs"),
        code,
    )
        .expect("Failed writing generated.rs");
}
