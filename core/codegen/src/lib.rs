use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Widget)]
pub fn widget_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_widget_macro(&ast)
}

fn impl_widget_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
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
            fn widget_name() -> &'static str {
                "#name"
            }
        }
    };
    gen.into()
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
    let gen = quote! {
        impl Appendable for #name {}
    };
    gen.into()
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
    let gen = quote! {
        impl Attributable for #name {}
    };
    gen.into()
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
    let gen = quote! {
        impl Classable for #name {}
    };
    gen.into()
}