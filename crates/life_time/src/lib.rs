mod longest;
mod excrept;
mod display;

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
        println!("result:{}", result);  // 此处输出和括号外输出的区别
    }
    // println!("result:{}", result);

}

#[cfg(test)]
mod tests {
    use excrept::ImportRxcerpt;

    use super::*;

    #[test]
    fn test_print1() {
        print1();
    }

    #[test]
    fn test_print2() {
        print2();
    }

    #[test]
    fn test_struct() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        // 获取第一个句子
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not found a '.' ");

        let i = ImportRxcerpt {
            part: first_sentence
        };
    }

    /// 可以省略生命周期标注
    fn test_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }


    #[test]
    fn test_impl_method1 {
        let a = "ddd";
        ImportRxcerpt::level(a);
    }


}