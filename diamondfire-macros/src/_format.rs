/// Currently unused.
/// May want to replace the current format implementation later on with this,
///  as it does not use trait objects.

use proc_macro::{
    TokenStream,
    TokenTree,
    Literal,
    Punct
};
use quote::{
    quote,
    quote_spanned
};
use rustc_parse_format::{
    Parser,
    ParseMode,
    Piece
};


pub(crate) fn format(input : TokenStream) -> TokenStream {

    let mut input = input.into_iter().peekable();

    // Construct a parser using the format arguemnt.
    let Some(string) = input.next() else {
        return quote!{ compile_error!("requires at least a format string argument") }.into();
    };
    let string = { match (string) {
        TokenTree::Literal(string) if let Ok(string) = string.str_value() => string,
        _ => { return quote_spanned!{ string.span().into() => compile_error!("format argument must be a string literal") }.into(); }
    } };
    let parser = Parser::new(&string, None, None, false, ParseMode::Format);

    // Parse arguments.
    while let Some(token) = input.next() {
        match (token) {
            TokenTree::Punct(punct) if (punct.as_char() == ',') => { },
            _ => {
                let msg = format!("expected `,`, found `{}`", token);
                return quote_spanned!{ token.span().into() => compile_error!(#msg) }.into();
            }
        }

        let Some(a) = input.next() else { break; };
        let b = { match (input.peek()) {
            TokenTree::Punct(punct) if (punct.as_char() == '=') => {

            },
            _ => None
        } };
    }

    let pieces = parser.map(|piece| { match (piece) {

        Piece::Lit(lit) => {
            quote!{ s.push_str(#lit); }
        },

        Piece::NextArgument(box arg) => todo!()

    } });

    quote::quote!{ {
        let mut s = String::new();
        #(#pieces)*
    } }.into()
}
