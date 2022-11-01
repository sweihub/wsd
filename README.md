wsd is an intuitive crate delivers What Simply Defined.


# Intutive File Class

Just a convenient wrapper to rust File, check the return value quickly as we did with C API, don't need to check the Result, and unwrap() etc.

The available file open flags
```rust
O_CREATE
O_TRUNCATE
O_RW
O_READ
O_WRITE
O_APPEND
```

```rust
using wsd::fs::*;

fn test() -> i32 {
    let mut f = File::new();
    if f.open("test.txt", O_CREATE | O_RW) != 0 {
        // check the error
        println!("Error: {}", f.error());
        return -1;
    }

    let data = "Hello World!";
    let n = f.write(data);
    if n < 0 {
        // write error
    }

    f.rewind();
    mut buf = [0; 4096];
    n = f.read(buf);
    if n > 0 {
        // read success        
    }

    // get file length
    let off = f.length();

    f.close();

    return 0;
} 

```