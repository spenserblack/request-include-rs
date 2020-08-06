extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

const ERROR: &str = "Invalid input";

#[proc_macro_hack]
pub fn include_str(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let input: String = match tokens.next() {
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
    let user_agent: Option<String> = match tokens.next() {
        Some(TokenTree::Punct(punct)) => {
            let punct = punct.as_char();
            if punct != ',' {
                panic!("Unexpected token: {:?}", punct);
            }
            match tokens.next() {
                Some(TokenTree::Literal(literal)) => {
                    let literal = literal.to_string();
                    let mut chars = literal.chars();
                    match (chars.next(), chars.next_back()) {
                        (Some('"'), Some('"')) => {},
                        _ => panic!(ERROR),
                    }
                    Some(chars.collect())
                },
                _ => panic!(ERROR),
            }
        },
        None => None,
        _ => panic!(ERROR),
    };
    match user_agent {
        None => {
            let body = reqwest::blocking::get(&input)
                .expect("Failed to get response")
                .text()
                .expect("Failed to parse response");
            let s = body.as_str();
            let token_stream = quote! {
                #s
            };
            TokenStream::from(token_stream)
        },
        Some(user_agent) => {
            let body = reqwest::blocking::Client::builder()
                .user_agent(&user_agent)
                .build()
                .expect("Failed to build client")
                .get(&input)
                .send()
                .expect("Failed to get response")
                .text()
                .expect("Failed to parse response to text");
            let s = body.as_str();
            let token_stream = quote! {
                #s
            };
            TokenStream::from(token_stream)
        },
    }
}
