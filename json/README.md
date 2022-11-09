# Native JSON for Rust

This crate provides real native JSON syntax for Rust, you can declare the JSON object natively like JavaScript dose.

## Example
```rust
use native_json::json;

fn test() {
    let mut object = json!{
        id: 2022,
        name: "Native JSON for Rust",
        array: [0,1,2,3,4,5,6,7,8,9],
        color: { red: 100, green: 110, blue: 120 },
        rect: { x: 10, y: 10, width: 100, height: 80},
        students: [
            { name: "Tom",  id: 202201 },
            { name: "John", id: 202202 },
            { name: "Jack", id: 202203 }
        ]
    }; 

    // Native access to JSON member
    let color = &object.color;
    println("color, red: {}, green: {}, blue: {}}", color.red, color.green, color.blue);

    for i in &object.array {
        // do something
    }

    object.rect.x += 10;
    object.rect.y += 20;
}

```
