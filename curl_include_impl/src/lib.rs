extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

const HARD_CODED: &str = "https://api.github.com/search/repositories?q=tetris&sort=stars&order=desc";

#[proc_macro_hack]
pub fn include_str(input: TokenStream) -> TokenStream {
    let body = reqwest::blocking::get(HARD_CODED)
        .unwrap()
        .text()
        .unwrap();
    let s = body.as_str();
    let token_stream = quote! {
        #s
    };
    TokenStream::from(token_stream)
}
