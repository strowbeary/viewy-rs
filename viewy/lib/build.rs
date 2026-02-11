extern crate heck;
extern crate quote;
extern crate scraper;
extern crate serde;
extern crate uuid;

use figment::{
    Figment,
    providers::{Format, Toml},
};
use heck::{ToKebabCase, ToPascalCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct IconPack {
    pub git: Option<String>,
    pub path: Option<String>,
    pub branch: Option<String>,
    pub prefix: Option<String>,
    pub stroked: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ViewyConfig {
    #[serde(rename = "icon-packs", default)]
    pub icon_packs: BTreeMap<String, IconPack>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LegacyConfig {
    #[serde(default)]
    pub icons: BTreeMap<String, IconPack>,
}

struct GeneratedPack {
    code: String,
    symbol_lookup_entries: Vec<(String, String)>,
}

fn project_root() -> PathBuf {
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("Failed reading CARGO_MANIFEST_DIR environment variable"),
    );

    for ancestor in manifest_dir.ancestors() {
        if ancestor.join("Viewy.toml").exists()
            || ancestor.join("viewy.toml").exists()
            || ancestor.join("viewy-icons.toml").exists()
        {
            return ancestor.to_path_buf();
        }
    }

    manifest_dir
}

fn load_icon_packs(root: &Path) -> BTreeMap<String, IconPack> {
    let modern_config = Figment::new()
        .merge(Toml::file(root.join("Viewy.toml")))
        .merge(Toml::file(root.join("viewy.toml")))
        .extract::<ViewyConfig>()
        .unwrap_or_default();

    if !modern_config.icon_packs.is_empty() {
        return modern_config.icon_packs;
    }

    Figment::new()
        .merge(Toml::file(root.join("viewy-icons.toml")))
        .extract::<LegacyConfig>()
        .map(|cfg| cfg.icons)
        .unwrap_or_default()
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_path = dst.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry_path, &destination_path)?;
        } else {
            fs::copy(&entry_path, &destination_path)?;
        }
    }

    Ok(())
}

fn path_from_icon(icons_folder_path: &Path, icon_name: &str) -> String {
    let file_content =
        fs::read_to_string(icons_folder_path.join(format!("{}.svg", icon_name.to_kebab_case())))
            .unwrap_or_default();

    scraper::Html::parse_fragment(&file_content)
        .select(&scraper::Selector::parse("svg").expect("Failed getting svg"))
        .map(|svg| svg.inner_html())
        .collect::<Vec<String>>()
        .join(" ")
}

fn resolve_icon_path(
    icon_pack_name: &str,
    pack: &IconPack,
    out_dir_path: &Path,
    root: &Path,
) -> Option<PathBuf> {
    if let Some(git_url) = &pack.git {
        let icons_path = out_dir_path.join(icon_pack_name);
        let branch_name = pack.branch.clone().unwrap_or_else(|| "main".to_string());
        let temp_folder = env::temp_dir().join(Uuid::new_v4().to_string());

        let status = Command::new("git")
            .args(["clone", "--depth", "1", "-b", &branch_name, git_url])
            .arg(&temp_folder)
            .status();

        if status.as_ref().is_err() || !status.expect("git clone status is missing").success() {
            eprintln!(
                "Viewy icon build warning: unable to clone {} (branch {}).",
                git_url, branch_name
            );
            return None;
        }

        let in_repo_path = temp_folder.join(pack.path.clone().unwrap_or_default());
        if !in_repo_path.exists() {
            eprintln!(
                "Viewy icon build warning: path {:?} not found in repository {}.",
                in_repo_path, git_url
            );
            let _ = fs::remove_dir_all(&temp_folder);
            return None;
        }

        if icons_path.exists() {
            let _ = fs::remove_dir_all(&icons_path);
        }

        if copy_dir_recursive(&in_repo_path, &icons_path).is_err() {
            eprintln!(
                "Viewy icon build warning: failed to copy icons from {:?} to {:?}.",
                in_repo_path, icons_path
            );
            let _ = fs::remove_dir_all(&temp_folder);
            return None;
        }

        let _ = fs::remove_dir_all(&temp_folder);
        Some(icons_path)
    } else {
        let local_icon_pack_path = root.join(pack.path.clone().unwrap_or_default());
        if local_icon_pack_path.exists() {
            Some(local_icon_pack_path)
        } else {
            eprintln!(
                "Viewy icon build warning: local icon path {:?} does not exist.",
                local_icon_pack_path
            );
            None
        }
    }
}

fn generate_icon_pack(
    icon_pack_name: &str,
    icon_pack_slug: &str,
    only_stroked: bool,
    icons_folder_path: &Path,
    icon_name_prefix: Option<String>,
) -> GeneratedPack {
    let icon_pack_name_ident = format_ident!("{}", icon_pack_name);

    let mut icon_names = fs::read_dir(icons_folder_path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .filter_map(|icon_file| icon_file.file_name().into_string().ok())
        .filter(|icon_file_name| icon_file_name.ends_with(".svg"))
        .filter_map(|icon_file_name| {
            icon_file_name
                .rsplit_once('.')
                .map(|(name, _)| name.to_string())
        })
        .collect::<Vec<String>>();

    icon_names.sort();
    icon_names.dedup();

    let mut symbol_lookup_entries = Vec::new();

    let icon_kind_enum_inner = icon_names.iter().map(|kind| {
        let svg_content = path_from_icon(icons_folder_path, kind);
        let symbol_id = format!("v-icon-{}-{}", icon_pack_slug.to_kebab_case(), kind.to_kebab_case());
        let symbol = format!(
            "<symbol id=\"{}\" viewBox=\"0 0 24 24\">{}</symbol>",
            symbol_id, svg_content
        );
        symbol_lookup_entries.push((symbol_id.clone(), symbol));

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
        let path = path_from_icon(icons_folder_path, kind);
        quote! {
            #icon_pack_name_ident::#kind_ident => #path,
        }
    });

    let symbol_id_arms = icon_names.iter().map(|kind| {
        let kind_ident = match &icon_name_prefix {
            None => format_ident!("{}", kind.to_pascal_case()),
            Some(prefix) => format_ident!("{}{}", prefix, kind.to_pascal_case()),
        };
        let symbol_id = format!(
            "v-icon-{}-{}",
            icon_pack_slug.to_kebab_case(),
            kind.to_kebab_case()
        );
        quote! {
            #icon_pack_name_ident::#kind_ident => #symbol_id,
        }
    });

    let configuration = if only_stroked {
        quote! {
            <Icon as crate::modifiers::Attributable>::set_attr(icon, "fill", "none");
            <Icon as crate::modifiers::Attributable>::set_attr(icon, "stroke", "currentColor");
        }
    } else {
        quote! {
            <Icon as crate::modifiers::Attributable>::set_attr(icon, "fill", "currentColor");
            <Icon as crate::modifiers::Attributable>::set_attr(icon, "stroke", "none");
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

            fn symbol_id(&self) -> &'static str {
                match self {
                    #(#symbol_id_arms)*
                }
            }

            fn configure(&self, icon: &mut Icon) {
                #configuration
            }
        }
    };

    GeneratedPack {
        code: code.to_string(),
        symbol_lookup_entries,
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    let root = project_root();
    for config_path in [
        root.join("Viewy.toml"),
        root.join("viewy.toml"),
        root.join("viewy-icons.toml"),
    ] {
        if config_path.exists() {
            println!("cargo:rerun-if-changed={}", config_path.display());
        }
    }

    let icon_packs = load_icon_packs(&root);
    let out_dir = env::var("OUT_DIR").expect("Failed reading OUT_DIR environment variable");
    let out_dir_path = Path::new(&out_dir);

    let mut code = String::new();
    let mut symbol_lookup_entries: Vec<(String, String)> = vec![];

    for (icon_pack_name, pack) in icon_packs {
        if pack.git.is_none() {
            let local_icon_pack_path = root.join(pack.path.clone().unwrap_or_default());
            if local_icon_pack_path.exists() {
                println!("cargo:rerun-if-changed={}", local_icon_pack_path.display());
            }
        }

        let Some(path) = resolve_icon_path(&icon_pack_name, &pack, out_dir_path, &root) else {
            continue;
        };

        let generated = generate_icon_pack(
            &icon_pack_name.to_upper_camel_case(),
            &icon_pack_name,
            pack.stroked.unwrap_or(true),
            &path,
            pack.prefix.clone(),
        );

        code.push_str(&generated.code);
        symbol_lookup_entries.extend(generated.symbol_lookup_entries);
    }

    if code.is_empty() {
        code.push_str("// No icon packs configured.\n");
    }

    code.push_str("\npub fn symbol_by_id(id: &str) -> Option<&'static str> {\n    match id {\n");

    for (symbol_id, symbol) in symbol_lookup_entries {
        code.push_str(&format!(
            "        {} => Some({}),\n",
            format!("{:?}", symbol_id),
            format!("{:?}", symbol)
        ));
    }

    code.push_str("        _ => None,\n    }\n}\n");

    fs::write(out_dir_path.join("icons.rs"), code).expect("Failed writing generated icons");

    println!("cargo:rustc-cfg=has_generated_feature");
}
