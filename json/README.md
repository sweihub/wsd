# Native JSON for Rust

This crate provides real native JSON syntax for Rust, you can declare the JSON object natively like JavaScript dose.

## Usage
Add dependencies to your Cargo.toml, `serde_json` is only needed if you want to stringify the JSON object.
```toml
[dependencies]
native-json = "1.0"
serde_json = "1.0.87"
```

## Example
```rust
use native_json::json;
use std::collections::HashMap;

fn main()
{
    let var = 123;
    let map = HashMap::from([ ("a", 1), ("b", 2), ("c", 3) ]);

    let mut t = json!{
        name: "native json",
        style: {
            color: "red",
            size: 12,
            bold: true
        },
        class: null,
        array: [5,4,3,2,1],
        vector: vec![1,2,3,4,5],
        hashmap: map,
        students: [
            {name: "John", age: 18},
            {name: "Jack", age: 21},
        ],
        rect: {x: 10, y: 10, width: 100, height: 50},
        sum: var + 10
    };

    // Native access
    t.rect.x += 10;
    t.rect.y += 20;

    // Debug
    println!("{:#?}", t);

    // Stringify
    let text = serde_json::to_string_pretty(&t).unwrap();
    println!("{}", text);

}
```
