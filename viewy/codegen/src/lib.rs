use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{LitStr, Meta};

#[proc_macro_derive(Widget, attributes(widget))]
pub fn widget_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // Extract name of the struct
    let name = &input.ident;

    // Parse `#[widget(script = "./script.js", style = "style.scss")]` attribute, if present
    let mut style_value: Option<String> = None;
    let mut script_value: Option<String> = None;

    for option in input.attrs.into_iter() {
        if let Meta::List(ref list) = option.meta {
            if list.path.is_ident("widget") {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("style") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        style_value = Some(s.value());
                        Ok(())
                    } else if meta.path.is_ident("script") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        script_value = Some(s.value());
                        Ok(())
                    } else {
                        Err(meta.error("unsupported attribute"))
                    }
                })
                .expect("Can parse attribute widget, check syntax");
            }
        }
    }

    let style_str = style_value.expect("style is a mandatory attribute in widget macro");
    // If script is not provided, use a default script file
    let script_str = script_value
        .map(|path| quote!(include_str!(#path)))
        .unwrap_or(quote!(""));
    let generated_code = quote! {
        use std::ops::{Deref, DerefMut};


        impl Into<Node> for #name {
            fn into(self) -> Node {
                let mut widget = self;
                widget.render();
                widget.node
            }
        }
       impl Into<Node> for &mut #name {
            fn into(self) -> Node {
                let widget = self;
                widget.render();
                widget.node.clone()
            }
        }

        impl Deref for #name {
            type Target = Node;

            fn deref(&self) -> &Self::Target {
                &self.node
            }
        }
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.node
            }
        }

        impl Widget for #name {
            const STYLE: &'static str = include_str!(#style_str);
            const SCRIPT: &'static str = #script_str;
            fn widget_name() -> &'static str {
                "#name"
            }

        }
    };
    generated_code.into()
}

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_component_macro(&ast)
}

fn impl_component_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
       impl Into<Node> for #name {
            fn into(self) -> Node {
                self.render()
            }
        }
    };
    generated_code.into()
}

#[proc_macro_derive(Appendable)]
pub fn appendable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_appendable_macro(&ast)
}

fn impl_appendable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Appendable for #name {}
    };
    generated_code.into()
}

#[proc_macro_derive(Attributable)]
pub fn attributable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_attributable_macro(&ast)
}

fn impl_attributable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Attributable for #name {}
    };
    generated_code.into()
}

#[proc_macro_derive(Classable)]
pub fn classable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_classable_macro(&ast)
}

fn impl_classable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Classable for #name {}
    };
    generated_code.into()
}
#[proc_macro_derive(Colorable)]
pub fn colorable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_colorable_macro(&ast)
}

fn impl_colorable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Colorable for #name {}
    };
    generated_code.into()
}

#[proc_macro_derive(Dimensionable)]
pub fn dimensionable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dimensionable_macro(&ast)
}

fn impl_dimensionable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Dimensionable for #name {}
    };
    generated_code.into()
}
