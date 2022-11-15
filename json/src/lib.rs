//!
//! # Native JSON for Rust
//! `Native json` brings to you the native JSON syntax for Rust.
//! ## Example of using JSON instance
//!```rust,no_run
//!use std::collections::HashMap;
//!use wsd::json::*;
//!
//!fn main()
//!{
//!    let mut json = json!{
//!        name: "native json",
//!        style: {
//!            color: "red",
//!            size: 12,
//!            bold: true,
//!            range: null
//!        },
//!        array: [5,4,3,2,1],
//!        vector: vec![1,2,3,4,5],
//!        hashmap: HashMap::from([("a", 1), ("b", 2), ("c", 3)]),
//!        students: [
//!            {name: "John", age: 18},
//!            {name: "Jack", age: 21},
//!        ],
//!    };
//!
//!    // Native access
//!    json.style.size += 1;
//!    json.students[0].age += 2;
//!
//!    // Stringify
//!    let text = json.stringify(4);
//!
//!    // Parse
//!    json.hashmap.clear();
//!    if let Err(e) = json.parse(&text) {
//!        println!("error: {}", e);
//!    }
//!
//!    println!("json.hashmap = {:#?}", json.hashmap);
//!}
//!```
//!## JSON as parameter
//!```rust
//!fn print_json<'t, T:wsd::json::JSON<'t>>(json: &T) {
//!    println!("{}", json.to_string());
//!}
//!```
//!## Example of using named JSON object
//!```rust,no_run
//!use native_json::json;
//!use serde::{Deserialize, Serialize};
//!use std::collections::HashMap;
//!
//!json!{ School {
//!    name: String,
//!    students: [
//!        { name: String, age: u16 },
//!        ...
//!    ],
//!    map: HashMap<String, String>,
//!    nullable: Option<String>
//!}}
//!
//!fn main()
//!{
//!    let mut school = School::new();
//!
//!    school.name = "MIT".to_string();
//!    school.map.insert("Tom".to_owned(), "Profile".to_owned());
//!
//!    // using initializer
//!    let mut john = School_students_item::new();
//!    john.name = "John".to_owned();
//!    john.age = 18;
//!    school.students.push(john);
//!
//!    // using struct
//!    let jack = School_students_item { name: "Jack".to_string(), age: 21 };
//!    school.students.push(jack);
//!
//!    // show
//!    println!("{:#?}", school);
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
    let parser = parse_macro_input!(input as Json); 
    let block = parser.get_block();
    // Show me the code
    // println!("XXXXXXXX\n{}", block);
    return TokenStream::from_str(block.as_str()).unwrap();
}
