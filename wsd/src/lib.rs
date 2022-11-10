pub mod fs;
pub mod http;
pub mod json;

#[cfg(test)]
mod tests {

    use crate::fs::*;

    #[test]
    fn test_open_failed() {
        let path = "test_open_failed";
        remove(path);

        let mut f = File::new();
        let ret = f.open(path, 0);
        assert!(ret != 0);

        // cargo test -- --nocapture
        // check the error
        let error = format!("{}", f.error());
        println!("Expected error: {}", error);
    }

    #[test]
    fn test_create_file() {
        let path = "test_create_file.txt";
        remove(path);

        let mut f = File::new();
        let mut ret = f.open(path, O_CREATE | O_RW);
        assert!(ret == 0);

        let data = "hello world\n";
        ret = f.write(data.as_bytes());
        assert!(ret == data.len() as i32);

        f.close();
        remove(path);
    }

    #[test]
    fn test_read_file() {
        let path = "test_read_file";
        remove(path);

        // ensure we have a file
        {
            let mut f = File::new();
            f.open(path, O_CREATE | O_TRUNCATE | O_WRITE);
            let data = "test";
            f.write(data);
            f.close();
        }

        let mut f = File::new();
        let mut n = f.open(path, O_RW | O_CREATE);
        assert!(n == 0);

        let data = "hello world, test read file!\n";
        n = f.write(data);
        assert!(n == data.len() as i32);

        let mut buf = [0; 4096];
        f.rewind();
        let n = f.read(&mut buf);
        assert!(n == data.len() as i32);

        // same content
        assert!(data.as_bytes() == &buf[0..(n as usize)]);

        f.close();
        remove(path);
    }

    #[test]
    fn test_file_size() {
        let path = "test_file_size.txt";
        remove(path);

        let data = "Hello world, test data.";
        let mut f = File::new();
        let n = f.open(path, O_RW | O_TRUNCATE | O_CREATE);
        assert!(n == 0);

        f.write(data);
        let off = f.length();
        assert!(off as usize == data.len());

        f.close();
        remove(path);
    }

    #[test]
    fn test_append_file() {
        let data1 = "hello\n";
        let data2 = "world\n";
        let path = "test_append_file.txt";
        remove(path);

        // append to file
        {
            let mut f = File::new();
            let mut n = f.open(path, O_CREATE | O_APPEND);
            assert!(n == 0);

            n = f.write(data1);
            assert!(n > 0);

            n = f.write(data2);
            assert!(n > 0);

            f.close();
        }

        // read and compare
        let mut f = File::new();
        let mut n = f.open(path, 0);
        assert!(n == 0);

        let target = data1.to_string() + data2;
        let mut buf = [0; 256];
        n = f.read(&mut buf);
        assert!(target.as_bytes() == &buf[0..(n as usize)]);

        f.close();
        n = remove(path);
        assert!(n == 0);
    }

    #[test]
    fn test_write_vector() {
        let path = "test_write_vector.txt";
        let mut f = File::new();
        let mut n = f.open(path, O_CREATE | O_WRITE);
        assert!(n == 0);

        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        n = f.write(&data);
        assert!(n == data.len() as i32);

        f.close();
        remove(path);
    }

    #[test]
    fn test_seek_and_read() {
        let path = "test_seek_and_read.txt";

        let mut f = File::new();
        let mut n = f.open(path, O_CREATE | O_RW);
        assert!(n == 0);

        let data1 = "0123456789";
        let data2 = "9876543210";
        f.write(data1);
        f.write(data2);
        n = f.rewind();
        assert!(n == 0);

        let mut off = f.length();
        assert!(off as usize == data1.len() + data2.len());

        off = f.seek(data1.len() as i64, SEEK_SET);
        assert!(off as usize == data1.len());

        let mut buf = [0; 32];
        n = f.read(&mut buf);
        assert!(n as usize == data2.len());

        // content must be correct
        assert!(data2.as_bytes() == &buf[0..n as usize]);

        f.close();
        remove(path);
    }
}
