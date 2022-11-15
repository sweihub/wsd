//!
//! # Native JSON for Rust
//! [Native json][json] brings to you the native JSON syntax for Rust.
//! ## Example of using JSON instance
//!```no_run,rust
//!use std::collections::HashMap;
//!use serde::{Deserialize, Serialize};
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
//!```rust,no_run
//!fn print_json<'t, T:wsd::json::JSON<'t>>(json: &T) {
//!    println!("{}", json.to_string());
//!}
//!```
//!## Example of using named JSON object
//!```rust
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
use std::{fs::read_to_string, path::Path};

pub use native_json::*;
pub use serde_json::Error;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct JSON;

impl JSON {
    /// Stringify a native-json object
    ///
    /// indent
    ///
    /// - 0 : output concise JSON string
    /// - N : pretty output with N spaces indentation
    pub fn stringify<T>(json: &T, indent: usize) -> String
    where
        T: ?Sized + Serialize,
    {
        let output;

        // concise
        if indent == 0 {
            match serde_json::to_string(json) {
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

        if let Err(e) = json.serialize(&mut ser) {
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

    pub fn parse<'a, T, TEXT>(json: &mut T, s: &'a TEXT) -> Result<(), serde_json::Error>
    where
        T: Deserialize<'a>,
        TEXT: AsRef<str>,
    {
        *json = serde_json::from_str(s.as_ref())?;
        return Ok(());
    }

    /// Return a concise JSON string
    pub fn to_string<T>(json: &T) -> String
    where
        T: ?Sized + Serialize,
    {
        match serde_json::to_string(&json) {
            Ok(s) => return s,
            Err(e) => {
                return format!("{{ error: \"{}\" }}", e.to_string());
            }
        }
    }

    /// Deserialize JSON from file
    pub fn read<T, F: AsRef<Path>>(json: &mut T, file: F) -> Result<&mut T, String>
    where
        T: DeserializeOwned,
    {
        let content;

        match read_to_string(file) {
            Ok(s) => {
                content = s;
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }

        if let Err(e) = JSON::parse(json, &content) {
            return Err(e.to_string());
        }

        return Ok(json);
    }

    /// Serialize JSON into file
    pub fn write<T, F: AsRef<Path>>(json: &T, file: F) -> std::io::Result<&T>
    where
        T: Serialize,
    {
        let content = JSON::stringify(json, 4);
        std::fs::write(file, content)?;
        return Ok(json);
    }
}

pub trait JSON2<'a>: Serialize + Deserialize<'a> {
    /// Parse JSON from string
    fn parse<T: AsRef<str>>(&mut self, s: &'a T) -> Result<(), serde_json::Error> {
        *self = serde_json::from_str(s.as_ref())?;
        Ok(())
    }

    /// Return a concise JSON string
    fn to_string(&self) -> String {
       return self.stringify(0);
    }

     /// Stringify a native-json object
    ///
    /// indent
    ///
    /// - 0 : output concise JSON string
    /// - N : pretty output with N spaces indentation
    fn stringify(&self, indent: usize) -> String   
    {
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
}

impl<'a, T> JSON2<'a> for T where T: Serialize + Deserialize<'a> {}
