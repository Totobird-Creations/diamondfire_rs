use proc_macro::TokenStream;


mod event;
#[proc_macro_attribute]
pub fn event(attr : TokenStream, item : TokenStream) -> TokenStream {
    event::event(attr, item)
}
