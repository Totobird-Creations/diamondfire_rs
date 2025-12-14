use proc_macro::TokenStream;
use syn::{
    ItemFn,
    Ident
};
use quote::{ quote, quote_spanned };
use rand::random;


#[proc_macro_attribute]
pub fn event(attr : TokenStream, item : TokenStream) -> TokenStream {
    let item = { match (syn::parse::<ItemFn>(item.clone())) {
        Ok(item) => item,
        Err(err) => { return err.into_compile_error().into(); }
    } };
    let attr = { match (syn::parse::<Ident>(attr)) {
        Ok(attr) => attr,
        Err(err) => { return err.into_compile_error().into(); }
    } };
    let     ident        = &item.sig.ident;
    let mut call_ident   = ident.clone();
    call_ident.set_span(attr.span());
    let     rn           = random::<u128>();
    let     module_ident = Ident::new(&format!("__PRIVATE__not_accessible_under_any_circumstance__{}_{:0>32x}", ident, rn), ident.span());
    let     event_ident  = Ident::new(&format!("DF_EVENT__{}__{}_{:0>32x}", attr, ident, rn), ident.span());

    let call = quote_spanned!{ attr.span() =>
        super::#call_ident()
    };
    quote!{
        #item

        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #module_ident {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[inline(never)]
            #[deprecated = "internal to diamondfire-macros"]
            #[unsafe(no_mangle)]
            extern "C" fn #event_ident() -> () {
                // TODO: Event-specific arguments
                #call
            }
        }
    }.into()
}


// #[proc_macro_attribute]
// pub fn string_enum(_attr : TokenStream, item : TokenStream) -> TokenStream {
//     let item = TokenStream2::from(item);
//     quote!{
//         #[derive(__private_diamondfire::__private_diamondfire_macros::__private__string_enum)]
//         #item
//     }.into()
// }

// #[doc(hidden)]
// #[expect(non_snake_case)]
// #[proc_macro_derive(__private__string_enum, attributes(string_enum_rename))]
// pub fn __private__string_enum(item : TokenStream) -> TokenStream {
//     if let Err(err) = syn::parse::<ItemEnum>(item.clone()) {
//         return err.into_compile_error().into();
//     }
//     quote!{ }.into()
// }
