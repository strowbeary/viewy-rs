extern crate heck;
extern crate quote;
extern crate scraper;

use std::env;
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

pub fn generate_icon_pack(icon_pack_name: &str, only_stroked: bool, icons_folder_path: &str, icon_name_prefix: Option<&str>) -> String {
    let icon_pack_name_ident = format_ident!("{}", icon_pack_name);
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
        let svg_content = path_from_icon(&path, kind);
        let svg_html = if only_stroked {
            format!(
                "<svg width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'>{}</svg>",
                svg_content
            )
        } else {
            format!(
                "<svg width='24' height='24' viewBox='0 0 24 24'>{}</svg>",
                svg_content
            )
        };
        let kind = match icon_name_prefix {
            None => format_ident!("{}", kind.to_pascal_case()),
            Some(prefix) => format_ident!("{}{}", prefix, kind.to_pascal_case()),
        };
        quote! {
            #[doc = #svg_html]
            #kind,
        }
    });

    let path_match_arms = icon_names.iter().map(|kind| {
        let kind_ident = match icon_name_prefix {
            None => format_ident!("{}", kind.to_pascal_case()),
            Some(prefix) => format_ident!("{}{}", prefix, kind.to_pascal_case()),
        };
        let path = path_from_icon(&path, kind);
        quote! {
            #icon_pack_name_ident::#kind_ident => #path,
        }
    });

    let configuration = if only_stroked {
        quote! {
            icon
                .set_attr("fill", "none")
                .set_attr("stroke", "currentColor")
        }
    } else {
        quote! {
            icon
                .set_attr("fill", "currentColor")
                .set_attr("stroke", "none")
        }
    };

    let code = quote! {

        /// Enum storing all the Octicons
        #[derive(PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
        pub enum #icon_pack_name_ident {
            #(#icon_kind_enum_inner)*
        }

        impl IconPack for #icon_pack_name_ident {
            fn path(&self) -> &'static str {
                match self {
                    #(#path_match_arms)*
                }
            }

            fn configure(&self, mut icon: Icon) -> Icon {
                #configuration
            }
        }
    };
    code.to_string()
}

fn main() {
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");
    let mut code = quote! {
         use crate::modifiers::DefaultModifiers;
    }
        .to_string();
    code += &generate_icon_pack("Lucide", true, "assets/lucide/icons", None);
    code += &generate_icon_pack("SimpleIcons", false, "assets/simple-icons/icons", Some("Icon"));
 //   code += &generate_icon_pack("ViewyIconPack", false, "assets/viewy-icon-pack", None);

    std::fs::write(
        Path::new(&env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable"))
            .join("icons.rs"),
        code,
    )
        .expect("Failed writing generated.rs");
}
