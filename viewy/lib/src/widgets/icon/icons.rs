use std::fmt::Debug;

use dyn_clone::{DynClone, clone_trait_object};

use crate::widgets::icon::Icon;

pub trait IconPack: DynClone + Send + Sync + Debug {
    fn path(&self) -> &'static str;
    fn symbol_id(&self) -> &'static str;
    fn configure(&self, icon: &mut Icon);
}

clone_trait_object!(IconPack);

impl IconPack for Box<dyn IconPack> {
    fn path(&self) -> &'static str {
        self.as_ref().path()
    }

    fn symbol_id(&self) -> &'static str {
        self.as_ref().symbol_id()
    }

    fn configure(&self, icon: &mut Icon) {
        self.as_ref().configure(icon)
    }
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

pub fn sprite_from_icon_ids<'a, I>(icon_ids: I) -> String
where
    I: IntoIterator<Item = &'a str>,
{
    let mut seen = std::collections::BTreeSet::new();
    let mut symbols = String::new();

    for icon_id in icon_ids {
        if seen.insert(icon_id.to_string()) {
            if let Some(symbol) = symbol_by_id(icon_id) {
                symbols.push_str(symbol);
            }
        }
    }

    if symbols.is_empty() {
        String::new()
    } else {
        format!(
            "<svg aria-hidden=\"true\" style=\"position:absolute;width:0;height:0;overflow:hidden\" xmlns=\"http://www.w3.org/2000/svg\">{symbols}</svg>"
        )
    }
}
