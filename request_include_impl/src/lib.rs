extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

const ERROR: &str = "Invalid input";

#[proc_macro_hack]
pub fn include_str(input: TokenStream) -> TokenStream {
    let input: String = match input.into_iter().next() {
        Some(TokenTree::Literal(literal)) => {
            let literal = literal.to_string();
            let mut chars = literal.chars();
            match (chars.next(), chars.next_back()) {
                (Some('"'), Some('"')) => {},
                _ => panic!(ERROR),
            }
            chars.collect()
        },
        _ => panic!(ERROR),
    };
    let body = reqwest::blocking::get(&input)
        .expect("Failed to get response")
        .text()
        .expect("Failed to parse response");
    let s = body.as_str();
    let token_stream = quote! {
        #s
    };
    TokenStream::from(token_stream)
}
