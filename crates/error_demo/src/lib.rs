mod anyhow;

use std::{fs::File, io::{Error, Read}};

fn open_file_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn open_file_expect() {
    let f = File::open("hello.txt").expect("无法打开文件");
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");
    let mut user_name_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match user_name_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_two() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_three() -> Result<String, Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
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

    // 测试错误传递
    #[test]
    fn test_read_username_from_file() {
        // assert_eq!("my name", read_username_from_file().unwrap());
        // assert_eq!("my name", read_username_from_file_two().unwrap());
        assert_eq!("my name", read_username_from_file_three().unwrap());
    }
}