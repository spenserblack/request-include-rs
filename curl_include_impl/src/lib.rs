extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_hack]
pub fn include_str(input: TokenStream) -> TokenStream {
    // unimplemented!("Get value");
    let val = "test";
    let token_stream = quote! {
        #val
    };
    TokenStream::from(token_stream)
}
