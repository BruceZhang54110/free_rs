mod longest;

fn print1() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        //let string2 = String::from("xyz");
        result = longest::fn_longest(string1.as_str(), string2);
    }
    println!("result:{}", result);
}

fn print2() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        //let string2 = String::from("xyz");
        result = longest::fn_longest(string1.as_str(), string2.as_str());
    }
    println!("result:{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print1() {
        print1();
    }

    #[test]
    fn test_print2() {
        print2();
    }


}