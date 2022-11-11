//!
//! # Native JSON for Rust
//! [Native json][json] brings to you the native JSON syntax for Rust.
//! ## Example
//!```rust
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
pub use native_json::*;
pub use serde_json::Error;

use serde::{Serialize, Deserialize};

pub trait JSON<'t>: Serialize  + Deserialize<'t> {
    /// Stringify a native-json object
    /// 
    /// indent
    /// 
    /// - 0 : output concise JSON string
    /// - N : pretty output with N spaces indentation
    fn stringify(&self, indent: usize) -> String {
        let output;

        // concise
        if indent == 0 {
            match serde_json::to_string(self) {
                Ok(s) => {
                    output = s;
                }
                Err(e) => {
                    return format!("{{ error : \"{}\" }}", e.to_string());
                }
            }
            return output;
        }

        // pretty
        let spaces = vec![' ' as u8; indent];
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        if let Err(e) = self.serialize(&mut ser) {
            return format!("{{ error : \"{}\" }}", e.to_string());
        }

        match String::from_utf8(ser.into_inner()) {
            Ok(s) => {
                output = s;
            }
            Err(e) => {
                return format!("{{ error : \"{}\" }}", e.to_string());
            }
        }

        return output;
    }

    /// Parse from a JSON string
    fn parse(&mut self, text: &'t String) -> Result<&mut Self, Error> {
        *self = serde_json::from_str(text.as_str())?;
        return Ok(self);
    }

    /// Return a concise JSON string
    fn to_string(&self) -> String {
        return self.stringify(0);
    }
}

// implement JSON for any compatible T
impl<'t, T> JSON<'t> for T
where
    T: Serialize + Deserialize<'t>
{

}
