use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;

pub fn derive_marker_trait(ast: &syn::DeriveInput, trait_name: &str) -> TokenStream {
    let name = &ast.ident;
    let trait_ident = format_ident!("{}", trait_name);
    quote! {
        impl #trait_ident for #name {}
    }
    .into()
}

pub fn derive_qualified_trait(
    ast: &syn::DeriveInput,
    trait_path: proc_macro2::TokenStream,
) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl #trait_path for #name {}
    }
    .into()
}

pub fn derive_box_stylable_bundle(
    ast: &syn::DeriveInput,
    viewy: &proc_macro2::TokenStream,
) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl #viewy::modifiers::Marginable for #name {}
        impl #viewy::modifiers::Borderable for #name {}
        impl #viewy::modifiers::Paddingable for #name {}
        impl #viewy::modifiers::Dimensionable for #name {}
        impl #viewy::modifiers::Scrollable for #name {}
        impl #viewy::modifiers::BoxStylable for #name {}
    }
    .into()
}
