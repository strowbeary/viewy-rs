extern crate cmd_lib;
extern crate download_git;
extern crate heck;
extern crate quote;
extern crate scraper;
extern crate serde;

use cmd_lib::run_cmd;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use heck::{ToKebabCase, ToPascalCase, ToUpperCamelCase};
use quote::format_ident;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::env::temp_dir;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct IconPack {
    pub git: Option<String>,
    pub path: Option<String>,
    pub branch: Option<String>,
    pub prefix: Option<String>,
    pub stroked: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub icons: HashMap<String, IconPack>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            icons: Default::default(),
        }
    }
}

fn path_from_icon(icons_folder_path: &PathBuf, icon_name: &str) -> String {
    let file_content = match std::fs::read_to_string(
        icons_folder_path.join(format!("{}.svg", icon_name.to_kebab_case())),
    ) {
        Ok(content) => content,
        // The icon doesn't exist for this size, so fall back to the other size.
        // Hopefully this doesn't loop forever.
        Err(_) => "".to_string(),
    };
    let inner_html = scraper::Html::parse_fragment(&file_content)
        .select(&scraper::Selector::parse("svg").expect("Failed getting svg"))
        .map(|svg| svg.inner_html())
        .collect::<Vec<String>>()
        .join(" ");
    inner_html
}

pub fn generate_icon_pack(
    icon_pack_name: &str,
    only_stroked: bool,
    icons_folder_path: &PathBuf,
    icon_name_prefix: Option<String>,
) -> String {
    let icon_pack_name_ident = format_ident!("{}", icon_pack_name);
    let path = icons_folder_path;
    let lucide_icons = std::fs::read_dir(&path).expect("Failed reading icons directory");

    let icon_names = lucide_icons
        .map(|icon_file| {
            icon_file
                .expect("Failed reading icons directory entry")
                .file_name()
                .into_string()
                .expect("Icon svg filename is not valid UTF-8")
        })
        .filter(|icon_file_name| icon_file_name.ends_with("svg"))
        .map(|icon_file_name| icon_file_name.split('.').collect::<Vec<&str>>()[0].to_string())
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
        let kind = match &icon_name_prefix {
            None => format_ident!("{}", kind.to_pascal_case()),
            Some(prefix) => format_ident!("{}{}", prefix, kind.to_pascal_case()),
        };
        quote! {
            #[doc = #svg_html]
            #kind,
        }
    });

    let path_match_arms = icon_names.iter().map(|kind| {
        let kind_ident = match &icon_name_prefix {
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

        /// Enum storing all the icons
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
    let path = env::var("OUT_DIR")
        .unwrap_or_default()
        .split("/target")
        .map(|part_str| part_str.to_string())
        .collect::<Vec<String>>()[0]
        .clone();
    let viewy_builded_project = Path::new(&path);

    let config = Figment::new()
        .merge(Toml::file(viewy_builded_project.join("viewy-icons.toml")))
        .extract::<Config>()
        .map_err(|err| {
            println!("Viewy config error {:?}", err);
            err
        })
        .unwrap_or_default();

    println!("{:?}", env::var("OUT_DIR"));

    let mut code = quote! {
         use crate::modifiers::DefaultModifiers;
    }
    .to_string();
    let out_dir = env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable");
    let out_dir_path = Path::new(&out_dir);

    for (icon_pack_name, pack) in config.icons {
        println!("{icon_pack_name} {:?}", pack);
        if let Some(git_url) = pack.git {
            let icons_path = out_dir_path.join(&icon_pack_name);
            let branch_name = pack.branch.unwrap_or("master".to_string());
            let temp_folder = temp_dir();
            let twd = temp_folder.join(Uuid::new_v4().to_string());
            let in_repo_path = twd.join(pack.path.unwrap_or_default());
            println!("{:?}", twd);
            let git_result = run_cmd! {
                mkdir -p $twd;
                git clone --verbose -b $branch_name $git_url $twd 2>&1;
            };
            println!("GIT RESULT {:?}", git_result);

            let move_result = run_cmd! {
                ls -a $twd 2>&1;
                mkdir -p $icons_path;
                cp -R $in_repo_path/. $icons_path 2>&1;
            };
            println!("MOVE RESULT {:?}", move_result);
            code += &generate_icon_pack(
                &icon_pack_name.to_upper_camel_case(),
                pack.stroked.unwrap_or(true),
                &icons_path,
                pack.prefix,
            );
        } else {
            let local_icon_pack_path = viewy_builded_project.join(pack.path.unwrap_or_default());
            println!("local_icon_pack_path {:?}", local_icon_pack_path);
            code += &generate_icon_pack(
                &icon_pack_name.to_upper_camel_case(),
                pack.stroked.unwrap_or(true),
                &local_icon_pack_path,
                pack.prefix,
            );
        }
    }

    std::fs::write(
        Path::new(&env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable"))
            .join("icons.rs"),
        code,
    )
    .expect("Failed writing generated.rs");

    println!("cargo:rustc-cfg=has_generated_feature")
}
