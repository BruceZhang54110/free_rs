use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_from_file() -> Result<String, std::io::Error> {
    let mut name = String::new();
    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)
}


fn read_from_file_dyn_error() -> Result<String, Box<dyn Error>> {
    let mut name = String::new();
    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)
}

fn read_from_file_any_how() -> anyhow::Result<String> {
    let mut name = String::new();
    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_read_from_file() {
        let name = read_from_file().unwrap();
        assert_eq!("bruce", name);
    }

    #[test]
    fn test_read_from_file_dyn_error() {
        let name = read_from_file_dyn_error().unwrap();
        assert_eq!("bruce", name);
    }

    #[test]
    fn test_read_from_file_any_how() {
        let name = read_from_file_any_how().unwrap();
        assert_eq!("bruce", name);
    }


}