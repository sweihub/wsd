wsd is an intuitive crate delivers What Simply Defined.

### Documents
https://docs.rs/wsd

# Intutive File Class

Just a convenient wrapper to rust File, check the return value quickly as we did with C API, don't need to check the Result, and unwrap() etc. 

## Methods
```rust
File::new()
File::open()
File::read()
File::write()
File::close()
File::error()
File::path()
File::seek()
File::position()
File::length()
File::is_none()
```

## Open flags
```rust
O_CREATE
O_TRUNCATE
O_RW
O_READ
O_WRITE
O_APPEND
O_NONBLOCK
```

## Seek flags
```rust
SEEK_SET
SEEK_CUR
SEEK_END
```


## Example

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
    let mut buf = [0; 4096];
    let n = f.read(&mut buf);
    if n > 0 {
        // success to read n bytes
    }

    // get file length
    let off = f.length();
    if off > 0 {

    }

    f.seek(256, SEEK_SET);
    f.write("more data");

    f.close();

    return 0;
} 

```

