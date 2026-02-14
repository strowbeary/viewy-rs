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

    let generated_code = quote! {
        impl ::viewy::InteractiveComponentMessage for #name {}
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
/// #[component(messages = CounterMessage)]
/// struct CounterComponent {
///     value: i32,
/// }
/// ```
#[proc_macro_derive(InteractiveComponent, attributes(component))]
pub fn interactive_component_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let handler_name = syn::Ident::new(
        format!("__viewy_interactive_component_handler_{}", name).as_str(),
        name.span(),
    );

    let mut message_type: Option<syn::Type> = None;
    for option in input.attrs.into_iter() {
        if let Meta::List(ref list) = option.meta {
            if list.path.is_ident("component") {
                list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("messages") {
                        let value = meta.value()?;
                        let message_ty: syn::Type = value.parse()?;
                        message_type = Some(message_ty);
                    }
                    Ok(())
                })
                .expect("Can't parse `component` attribute, check syntax");
            }
        }
    }

    let message_type =
        message_type.expect("`messages` is a mandatory argument in #[component(...)]");

    let generated_code = quote! {
        #[allow(non_snake_case)]
        fn #handler_name(
            form: &::viewy::bindings::rocket::component::InteractiveComponentEventForm,
        ) -> Result<::viewy::node::Node, String> {
            let raw_state = form
                .get("_v_component_state")
                .ok_or_else(|| "Missing `_v_component_state` form field".to_string())?;
            let component: #name =
                ::viewy::bindings::rocket::component::decode_component_state(raw_state)?;

            let raw_message = form
                .get("_v_component_msg")
                .ok_or_else(|| "Missing `_v_component_msg` form field".to_string())?;
            let message: #message_type =
                ::viewy::bindings::rocket::component::decode_component_message(raw_message)?;

            let next_component =
                <#name as ::viewy::InteractiveComponent>::on_message(
                    component,
                    message,
                );
            let content =
                ::viewy::bindings::rocket::component::interactive_component_content(
                    next_component,
                )?;
            Ok(content.into())
        }

        ::viewy::inventory::submit! {
            ::viewy::bindings::rocket::component::InteractiveComponentRegistration::new(
                concat!(module_path!(), "::", stringify!(#name)),
                #handler_name,
            )
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
