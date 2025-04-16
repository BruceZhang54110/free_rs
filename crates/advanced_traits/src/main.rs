pub trait Iterator1 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    
}
pub trait Iterator2<T> {

    fn next(&mut self) -> Option<T>;
    
}

struct Counter {}

impl Iterator1 for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
    
}
/// 会报错，不允许为同一个类型实现多个 trait
// impl Iterator1 for Counter {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
    
// }

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
    
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
    
}



fn main() {
    println!("Hello, world!");
}
