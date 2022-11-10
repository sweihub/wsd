//!# Native JSON for Rust
//!
//!This crate provides real native JSON syntax for Rust, you can declare the JSON object natively like JavaScript dose.
//!
//!## Usage
//!Add dependencies to your Cargo.toml, `serde_json` is only needed if you want to stringify the JSON object.
//!```toml
//![dependencies]
//!native-json = "1.0"
//!serde_json = "1.0.87"
//!```
//!
//!## Example
//!```rust
//!use native_json::json;
//!use std::collections::HashMap;
//!
//!fn main()
//!{
//!    let var = 123;
//!    let map = HashMap::from([ ("a", 1), ("b", 2), ("c", 3) ]);
//!
//!    let mut t = json!{
//!        name: "native json",
//!        style: {
//!            color: "red",
//!            size: 12,
//!            bold: true
//!        },
//!        class: null,
//!        array: [5,4,3,2,1],
//!        vector: vec![1,2,3,4,5],
//!        hashmap: map,
//!        students: [
//!            {name: "John", age: 18},
//!            {name: "Jack", age: 21},
//!        ],
//!        rect: {x: 10, y: 10, width: 100, height: 50},
//!        sum: var + 10
//!    };
//!
//!    // Native access
//!    t.rect.x += 10;
//!    t.rect.y += 20;
//!
//!    // Debug
//!    println!("{:#?}", t);
//!
//!    // Stringify
//!    let text = serde_json::to_string_pretty(&t).unwrap();
//!    println!("{}", text);
//!
//!}
//!```
extern crate proc_macro;
mod json;

use json::*;
use std::str::FromStr;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Declare a native JSON object, native access to members.
///```rust,no_run
///use native_json::json;
///fn test() {
///    let object = json!{
///        name: "native json",
///        rect: { x: 10, y: 20, width: 100, height: 50},
///        list: [0,1,2,3,4,5,6,7,8,9]
///    };
///    println!("name: {}", object.name);
///}
///```
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as Json); 
    let prototypes = root.get_pototypes();
    let code = root.get_code();
    let block = format!("{{ {}\n{} }}", prototypes, code);
    return TokenStream::from_str(block.as_str()).unwrap();    
}
