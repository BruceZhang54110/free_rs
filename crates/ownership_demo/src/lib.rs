fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_i32(value: i32) {
    println!("{}", value);
}

// 返回一个String 类型
fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_takes_ownership() {
        let s = String::from("hello");
        takes_ownership(s);
        //println!("s:{}", s);
    }

    #[test]
    fn test_takes_ownership_i32() {
        let value = 12;
        takes_ownership_i32(value);
        println!("value:{}", value);
    }

    #[test]
    fn test_gives_ownership() {
        let s = gives_ownership();
        takes_ownership(s);
        println!("s:{}", s);
    }

}