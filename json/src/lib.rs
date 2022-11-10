extern crate proc_macro;
mod json;

use json::*;
use std::str::FromStr;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Json); 
    let prototypes = root.get_pototypes();
    let code = root.get_code();
    let block = format!("{{ {}\n{} }}", prototypes, code);
    return TokenStream::from_str(block.as_str()).unwrap();    
}
