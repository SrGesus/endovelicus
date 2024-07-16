extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Ident};


#[proc_macro_derive(OptionalModel)]
pub fn derive_optional_model(item:  TokenStream) -> TokenStream {
  let mut item: DeriveInput = parse_macro_input!(item);
  item.ident = Ident::new("OptionalModel", item.span());

  if let Data::Struct(ref mut data) = item.data {
    for field in &mut data.fields {
      let ty = &field.ty; // Using field.ty directly panics
      field.ty = parse_quote! { Option<#ty> };
    }
  } else {
    syn::Error::new(item.span(), "expected struct definition.");
  }
  quote! {
    #item
  }.into()
}
