//!The intuitive file system module
//!
//! Demo of the intuitive File, call methods like C API convention, check the returned integer for status, 0 means success, negative value means error.
//!
//! ```rust,no_run
//!use wsd::fs::*;
//!
//!fn test() -> i32 {
//!    let mut f = File::new();
//!     if f.open("test.txt", O_CREATE | O_RW) != 0 {
//!         // check the error
//!         println!("Error: {}", f.error());
//!        return -1;
//!    }
//!
//!    let data = "Hello World!";
//!    let n = f.write(data);
//!    if n < 0 {
//!        // write error
//!    }
//!
//!    f.rewind();
//!    let mut buf = [0; 4096];
//!    let n = f.read(&mut buf);
//!    if n > 0 {
//!        // success to read n bytes
//!    }
//!
//!    f.seek(256, SEEK_SET);
//!    f.write("more data");
//!
//!    f.close();
//!
//!    return 0;
//!}
//!
//!```
use std::io::prelude::*;
use std::path::Path;

/// Create and open the file
pub const O_CREATE: u32 = 1 << 1;
/// Append only
pub const O_APPEND: u32 = 1 << 2;
/// Non blocking to [`File::write`]
pub const O_NONBLOCK: u32 = 1 << 3;
/// Read only
pub const O_READ: u32 = 1 << 4;
/// Write only
pub const O_WRITE: u32 = 1 << 5;
/// Read and write
pub const O_RW: u32 = O_READ | O_WRITE;
/// Causes the file to be truncated if it exists
pub const O_TRUNCATE: u32 = 1 << 6;

/// Seek to absolute position
pub const SEEK_SET: i32 = 1;
/// Seek to relative position from current
pub const SEEK_CUR: i32 = 2;
/// Seek to relative position from end
pub const SEEK_END: i32 = 3;

#[allow(non_camel_case_types)]
type int = i32;

/// Create directories recursively
pub fn mkdir<P: AsRef<Path>>(path: P) -> int {
    if let Err(_) = std::fs::create_dir_all(path) {
        return -1;
    }
    return 0;
}

/// Remove a file
pub fn remove<T: AsRef<Path>>(path: T) -> int {
    if let Err(_) = std::fs::remove_file(path) {
        return -1;
    }
    return 0;
}

/// Intuitive File
pub struct File {
    pod: Option<std::fs::File>,
    path: String,
    flags: u32,
    error: std::io::Error,
}

impl File {
    /// Returns a new File instance
    pub fn new() -> Self {
        return File {
            pod: None,
            path: "".to_string(),
            flags: 0,
            error: std::io::Error::new(std::io::ErrorKind::Other, ""),
        };
    }

    /// Open a file with given flags
    pub fn open<T: AsRef<str>>(&mut self, path: T, flags: u32) -> int {
        let mut options = std::fs::File::options();

        self.pod = None;
        self.path = path.as_ref().to_string();

        self.flags = flags;
        if flags == 0 {
            self.flags |= O_READ;
        }

        options.create(self.flags & O_CREATE != 0);
        options.append(self.flags & O_APPEND != 0);
        options.read(self.flags & O_READ != 0);
        options.write(self.flags & O_WRITE != 0);
        options.truncate(self.flags & O_TRUNCATE != 0);

        let ret = options.open(path.as_ref());
        match ret {
            Ok(f) => {
                self.pod = Some(f);
            }
            Err(e) => {
                self.error = e;
                return -1;
            }
        }

        return 0;
    }

    /// Simply drop the inner file descriptor
    pub fn close(&mut self) {
        self.pod = None;
    }

    /// Returns the path of the file
    pub fn path(&self) -> &String {
        return &self.path;
    }

    /// Returns the last error of calls
    pub fn error(&self) -> &std::io::Error {
        return &self.error;
    }

    /// Write all data unless [`O_NONBLOCK`] flag was set
    pub fn write<Buffer: AsRef<[u8]>>(&mut self, data: Buffer) -> int {
        let mut i = 0;
        let buf = data.as_ref();
        let n = buf.len() as i32;

        if self.is_none() {
            return -1;
        }

        let nb = self.flags & O_NONBLOCK != 0;
        let mut fd = self.fd();

        while i < n {
            let off = i as usize;
            let ret = fd.write(&buf[off..]);
            match ret {
                Ok(n) => i += n as i32,
                Err(e) => {
                    self.error = e;
                    break;
                }
            }
            if nb {
                break;
            }
        }

        return i;
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> i32 {
        if self.is_none() {
            return -1;
        }

        match self.fd().read_to_end(buf)  {
            Ok(n) => {
                return  n as i32;
            },
            Err(e) =>  {
                self.error = e;
            }
        }

        return -1;
    }

    /// Read data into buffer
    pub fn read(&mut self, buf: &mut [u8]) -> int {
        if self.is_none() {
            return -1;
        }

        let mut i = 0;
        let mut fd = self.fd();

        let ret = fd.read(buf);
        match ret {
            Ok(n) => {
                i = n as i32;
            }
            Err(e) => {
                self.error = e;
            }
        }

        return i;
    }

    /// Flush the file
    pub fn flush(&mut self) -> int {
        if self.is_none() {
            return -1;
        }

        if let Err(e) = self.fd().flush() {
            self.error = e;
            return -1;
        }

        return 0;
    }

    /// Seek to a position
    /// * `offset` - relative position
    /// * `whence` - One of: [`SEEK_SET`], [`SEEK_CUR`], [`SEEK_END`]
    pub fn seek(&mut self, offset: i64, whence: int) -> i64 {
        if self.is_none() {
            return -1;
        }

        let w;
        let mut off = -1;

        match whence {
            SEEK_SET => {
                w = std::io::SeekFrom::Start(offset as u64);
            }
            SEEK_CUR => {
                w = std::io::SeekFrom::Current(offset);
            }
            SEEK_END => {
                w = std::io::SeekFrom::End(offset);
            }
            _ => {
                return off;
            }
        }

        let ret = self.fd().seek(w);
        match ret {
            Ok(n) => {
                off = n as i64;
            }
            Err(e) => {
                self.error = e;
            }
        }

        return off;
    }

    /// Reset the position
    pub fn rewind(&mut self) -> int {
        if self.seek(0, SEEK_SET) < 0 {
            return -1;
        }
        return 0;
    }

    /// Returns the current position
    pub fn position(&mut self) -> i64 {
        return self.seek(0, SEEK_CUR);
    }

    /// Check if inner file descriptor is none
    pub fn is_none(&self) -> bool {
        return self.pod.is_none();
    }

    /// Returns a reference to inner file descriptor
    pub fn fd(&mut self) -> Box<&mut std::fs::File> {
        return Box::new(self.pod.as_mut().unwrap());
    }
}
