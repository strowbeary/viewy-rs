extern crate heck;
extern crate quote;
extern crate scraper;

use std::env;
use std::path::Path;

use heck::ToKebabCase;
use heck::ToPascalCase;
use quote::format_ident;
use quote::quote;

fn path_from_icon(icon_name: &str) -> String {
    let file_path = format!("lucide/icons/{}.svg", icon_name.to_kebab_case());
    println!("{}", file_path);
    let file_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        // The icon doesn't exist for this size, so fall back to the other size.
        // Hopefully this doesn't loop forever.
        Err(_) => "".to_string(),
    };
    let inner_html = scraper::Html::parse_fragment(&file_content)
        .select(&scraper::Selector::parse("svg").expect("Failed getting svg"))
        .map(|svg| {
            svg.inner_html()
        })
        .collect::<Vec<String>>()
        .join(" ");
    inner_html
}

fn main() {
    println!("cargo:rerun-if-changed={}", "lucide/icons");
    println!("cargo:rerun-if-changed={}", "build.rs");
    let lucide_icons =
        std::fs::read_dir("lucide/icons").expect("Failed reading lucide/icons directory");

    let icon_names = lucide_icons
        .map(|lucide_icon| {
            let icon_file_name = lucide_icon
                .expect("Failed reading lucide/icons directory entry")
                .file_name()
                .into_string()
                .expect("Icon svg filename is not valid UTF-8");
            icon_file_name.split('.').collect::<Vec<&str>>()[0].to_string()
        })
        .collect::<Vec<String>>();

    let icon_kind_enum_inner = icon_names.iter().map(|kind| {
        let kind = format_ident!("{}", kind.to_pascal_case());
        quote! {
            #kind,
        }
    });

    let path_match_arms = icon_names.iter().map(|kind| {
        let kind_ident = format_ident!("{}", kind.to_pascal_case());
        let path = path_from_icon(kind);
        quote! {
            Lucide::#kind_ident => #path,
        }
    });

    let code = quote! {
        /// Enum storing all the Octicons
        #[derive(PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
        pub enum Lucide {
            #(#icon_kind_enum_inner)*
        }

        impl IconPack for Lucide {
            fn path(&self) -> &'static str {
                match self {
                    #(#path_match_arms)*
                }
            }
        }
    };

    std::fs::write(
        Path::new(&env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable"))
            .join("icons.rs"),
        code.to_string(),
    )
    .expect("Failed writing generated.rs");
}
