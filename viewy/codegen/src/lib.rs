use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{format_ident, quote};
use syn;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{LitStr, Meta, Token};

fn viewy_path() -> proc_macro2::TokenStream {
    match crate_name("viewy") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote!(::#ident)
        }
        Err(_) => quote!(::viewy),
    }
}

fn build_interactive_registration(
    viewy: &proc_macro2::TokenStream,
    component_ty: &syn::Type,
    handler_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        #[allow(non_snake_case)]
        fn #handler_name(
            form: &#viewy::bindings::rocket::component::InteractiveComponentEventForm,
        ) -> Result<#viewy::node::Node, String> {
            let raw_state = form
                .get("_v_component_state")
                .ok_or_else(|| "Missing `_v_component_state` form field".to_string())?;
            let component: #component_ty =
                #viewy::bindings::rocket::component::decode_component_state(raw_state)?;

            let raw_message = form
                .get("_v_component_msg")
                .ok_or_else(|| "Missing `_v_component_msg` form field".to_string())?;
            let message: <#component_ty as #viewy::InteractiveComponent>::Message =
                #viewy::bindings::rocket::component::decode_component_message(raw_message)?;
            let component_id = form
                .get("_v_component_id")
                .ok_or_else(|| "Missing `_v_component_id` form field".to_string())?;
            let current_version = form
                .get("_v_component_version")
                .and_then(|raw| raw.parse::<u64>().ok())
                .unwrap_or(0);
            let next_version = current_version.saturating_add(1);

            let next_component =
                <#component_ty as #viewy::InteractiveComponent>::on_message(component, message);
            #viewy::bindings::rocket::component::interactive_component_root_with_id_and_version(
                component_id,
                next_version,
                next_component,
            )
        }

        #viewy::inventory::submit! {
            #viewy::bindings::rocket::component::InteractiveComponentRegistration::new(
                <#component_ty as #viewy::InteractiveComponentMetadata>::REGISTRATION_NAME,
                #handler_name,
            )
        }
    }
}

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
                    }
                    if meta.path.is_ident("script") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        script_value = Some(s.value());
                    }
                    Ok(())
                })
                .expect("Can't parse attribute widget, check syntax");
            }
        }
    }

    let style_str = style_value.expect("style is a mandatory attribute in widget macro");

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
            fn widget_name() -> &'static str {
                stringify!(#name)
            }

        }

        ::inventory::submit! {
            crate::core::widget::WidgetStyleRegistration::new(
                <#name as Widget>::STYLE,
            )
        }
    };
    generated_code.into()
}

#[proc_macro_derive(InteractiveComponentMessage)]
pub fn interactive_component_message_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let viewy = viewy_path();

    let generated_code = quote! {
        impl #viewy::InteractiveComponentMessage for #name {}
    };

    generated_code.into()
}

/// Derive macro that registers an interactive component in Viewy's
/// Rocket single-route registry.
///
/// Usage:
/// ```rust
/// use rocket::serde::{Deserialize, Serialize};
/// use viewy::prelude::*;
///
/// #[derive(Serialize, Deserialize, InteractiveComponentMessage)]
/// #[serde(crate = "rocket::serde")]
/// enum CounterMessage {
///     Increment,
/// }
///
/// #[derive(Serialize, Deserialize, InteractiveComponent)]
/// #[serde(crate = "rocket::serde")]
/// struct CounterComponent {
///     value: i32,
/// }
/// ```
#[proc_macro_derive(InteractiveComponent, attributes(component))]
pub fn interactive_component_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    if !matches!(input.data, syn::Data::Struct(_)) {
        return syn::Error::new_spanned(
            &input.ident,
            "`#[derive(InteractiveComponent)]` can only be used on structs.\n\
Help: move the interactive state into a struct and implement `InteractiveComponent` on it.",
        )
        .to_compile_error()
        .into();
    }

    if let Some(attr) = input
        .attrs
        .iter()
        .find(|attr| matches!(&attr.meta, Meta::List(list) if list.path.is_ident("component")))
    {
        return syn::Error::new_spanned(
            attr,
            "The `#[component(...)]` attribute is no longer used by `InteractiveComponent`.\n\
Help: remove this attribute and declare only `type Message = ...` in your `impl InteractiveComponent`.",
        )
        .to_compile_error()
        .into();
    }

    let name = &input.ident;
    if !input.generics.params.is_empty() {
        return syn::Error::new_spanned(
            &input.generics,
            "`#[derive(InteractiveComponent)]` does not support generic component definitions.\n\
Help: register one or more concrete instantiations at module level, for example:\n\
`viewy::register_interactive_component!(MyComponent<Book>, MyComponent<Movie>);`",
        )
        .to_compile_error()
        .into();
    }
    let viewy = viewy_path();
    let handler_name = format_ident!("__viewy_interactive_component_handler_{}", name);
    let component_ty: syn::Type = syn::parse_quote!(#name);
    let generated_code = {
        let registration = build_interactive_registration(&viewy, &component_ty, &handler_name);
        quote! {
            impl #viewy::InteractiveComponentMetadata for #name {
                const REGISTRATION_NAME: &'static str =
                    concat!(module_path!(), "::", stringify!(#name));
            }

            #registration
        }
    };

    generated_code.into()
}

/// Register concrete interactive component types at compile time.
///
/// This is useful for generic component definitions where derive cannot infer
/// which concrete instantiations must be registered.
///
/// Usage:
/// ```rust
/// viewy::register_interactive_component!(
///     MyGenericComponent<Book>,
///     MyGenericComponent<Movie>,
/// );
/// ```
#[proc_macro]
pub fn register_interactive_component(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<syn::Type, Token![,]>::parse_terminated;
    let component_types = match parser.parse(input) {
        Ok(types) => types,
        Err(err) => return err.to_compile_error().into(),
    };

    if component_types.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "`register_interactive_component!` expects at least one concrete type.\n\
Help: e.g. `viewy::register_interactive_component!(CounterComponent);`",
        )
        .to_compile_error()
        .into();
    }

    let viewy = viewy_path();
    let mut registrations = Vec::with_capacity(component_types.len());
    for (index, component_ty) in component_types.iter().enumerate() {
        let handler_name =
            format_ident!("__viewy_interactive_component_registered_handler_{}", index);
        let registration = build_interactive_registration(&viewy, component_ty, &handler_name);
        registrations.push(quote! {
            const _: () = {
                impl #viewy::InteractiveComponentMetadata for #component_ty {
                    const REGISTRATION_NAME: &'static str = stringify!(#component_ty);
                }
                #registration
            };
        });
    }

    quote! {
        #(#registrations)*
    }
    .into()
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

#[proc_macro_derive(Cardifiable)]
pub fn cardifiable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_cardifiable_macro(&ast)
}

fn impl_cardifiable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl Cardifiable for #name {}
    };
    generated_code.into()
}
