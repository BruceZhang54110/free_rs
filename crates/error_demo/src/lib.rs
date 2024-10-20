use std::fs::File;

fn open_file_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn open_file_expect() {
    let f = File::open("hello.txt").expect("无法打开文件");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_unwrap() {
        open_file_unwrap();
    }

    #[test]
    fn test_open_file_expect() {
        open_file_expect();
    }
}