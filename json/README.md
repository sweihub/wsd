# Native JSON for Rust

This crate provides native JSON syntax for Rust, you can declare the JSON object natively like JavaScript dose.

Note: This crate is just a crude proc-maco (compiler plugin) for Rust, for more features, please refer to [wsd::json](https://crates.io/crates/wsd)

## Usage
Add dependencies to your Cargo.toml, `serde_json` is only needed if you want to stringify the JSON object.
```toml
[dependencies]
native-json = "1.0"
serde = "1.0"
serde_json = "1.0"
```

## Example
```rust
use native_json::json;
use std::collections::HashMap;

fn main()
{
    let mut json = json!{
        name: "native json",
        style: {
            color: "red",
            size: 12,
            bold: true,
            range: null
        },
        array: [5,4,3,2,1],
        vector: vec![1,2,3,4,5],
        hashmap: HashMap::from([ ("a", 1), ("b", 2), ("c", 3) ]);,
        students: [
            {name: "John", age: 18},
            {name: "Jack", age: 21},
        ],
    };

    // Native access
    json.style.size += 1;
    json.students[0].age += 2;

    // Debug
    println!("{:#?}", t);

    // Stringify
    let text = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", text);
}
```
