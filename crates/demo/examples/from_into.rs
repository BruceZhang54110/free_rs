struct Student {
    name: String,
}

impl From<String> for Student {
    fn from(value: String) -> Self {
        Student { name: value }
    }
}


fn main() {

    
    let _s = "Rust".to_string();


    // impl Into<String> for &str
    let s1: String = "Rust".into();

    //impl From<&str> for String	
    let s2 = String::from("Rust");

    let a: String = s2.into();

    //println!("s1 == s2, {}", s1 == s2);

    let _s1 = Student::from("zhangsan".to_string());

    let _s2: Student = "zhangsan".to_string().into();

}