
use std::env;
use std::fmt::format;
use std::path::{Path, PathBuf};

use heck::ToKebabCase;
use heck::ToPascalCase;
use quote::format_ident;
use quote::quote;


fn path_from_icon(icons_folder_path: &PathBuf, icon_name: &str) -> String {
    let file_content = match std::fs::read_to_string(icons_folder_path.join(format!("{}.svg", icon_name.to_kebab_case()))) {
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

pub fn generate_icon_pack(icon_pack_name: &str, icons_folder_path: &str) {

    println!("cargo:rerun-if-changed={}", icons_folder_path);
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var"))
        .join(icons_folder_path);
    let lucide_icons =
        std::fs::read_dir(&path).expect("Failed reading icons directory");

    let icon_names = lucide_icons
        .map(|lucide_icon| {
            let icon_file_name = lucide_icon
                .expect("Failed reading icons directory entry")
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
        let path = path_from_icon(&path, kind);
        quote! {
            #icon_pack_name::#kind_ident => #path,
        }
    });

    let code = quote! {
        /// Enum storing all the Octicons
        #[derive(PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
        pub enum #icon_pack_name {
            #(#icon_kind_enum_inner)*
        }

        impl IconPack for #icon_pack_name {
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