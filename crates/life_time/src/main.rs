mod longest;

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        //let string2 = String::from("xyz");
        result = longest::fn_longest(string1.as_str(), string2);
    }
    println!("result:{}", result);
}
