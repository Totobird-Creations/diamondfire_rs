use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{ ItemEnum, ItemFn };
use quote::quote;


#[proc_macro_attribute]
pub fn event(_attr : TokenStream, item : TokenStream) -> TokenStream {
    if let Err(err) = syn::parse::<ItemFn>(item.clone()) {
        return err.into_compile_error().into();
    }
    item
}


#[proc_macro_attribute]
pub fn string_enum(_attr : TokenStream, item : TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    quote!{
        #[derive(__private_diamondfire::__private_diamondfire_macros::__private__string_enum)]
        #item
    }.into()
}

#[doc(hidden)]
#[expect(non_snake_case)]
#[proc_macro_derive(__private__string_enum, attributes(string_enum_rename))]
pub fn __private__string_enum(item : TokenStream) -> TokenStream {
    if let Err(err) = syn::parse::<ItemEnum>(item.clone()) {
        return err.into_compile_error().into();
    }
    quote!{ }.into()
}
