extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Ident};

#[proc_macro_derive(IntoActive)]
pub fn derive_into_active(item: TokenStream) -> TokenStream {
  let mut item: DeriveInput = parse_macro_input!(item);
  let Data::Struct(ref mut data) = item.data else {
    return syn::Error::new(item.span(), "Expected a struct definition")
      .to_compile_error()
      .into();
  };
  let idents = data.fields.iter().map(|f| f.ident.clone());
  let ident = item.ident;

  quote! {
    impl #ident {
      pub fn into_active(self) -> ActiveModel {
        ActiveModel {
          #(
            #idents: sea_orm::Set(self.#idents)
          ),*
        }
      }
    }
  }
  .into()
}
#[proc_macro_derive(OptionalModel)]
pub fn derive_optional_model(item: TokenStream) -> TokenStream {
  let mut item: DeriveInput = parse_macro_input!(item);
  let ident = Ident::new("OptionalModel", item.span());
  item.ident = ident.clone();

  // Remove struct attributes that cause errors
  item
    .attrs
    .retain(|attr| attr.parse_args::<proc_macro2::TokenStream>().is_err());

  let Data::Struct(ref mut data) = item.data else {
    return syn::Error::new(item.span(), "Expected a struct definition")
      .to_compile_error()
      .into();
  };

  for field in &mut data.fields {
    let ty = &field.ty; // Using field.ty in quote! directly panics
                        // Remove struct attributes that cause errors
    field
      .attrs
      .retain(|attr| attr.parse_args::<proc_macro2::TokenStream>().is_err());
    field.ty = parse_quote! { Option<#ty> };
  }
  // There's a need to bring data's lifetime to an end because it's a mutable reference
  let idents: Vec<_> = data.fields.iter().map(|f| f.ident.clone()).collect();

  quote! {
    #[derive(serde::Serialize, serde::Deserialize)]
    #item

    impl #ident {
      pub fn into_active(self) -> ActiveModel {
        ActiveModel {
          #(
            #idents: match self.#idents {
              Some(#idents) => sea_orm::Set(#idents),
              None => sea_orm::NotSet
            }
          ),*
        }
      }
    }
  }
  .into()
}
