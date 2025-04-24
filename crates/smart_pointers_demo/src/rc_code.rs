struct Example;

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }

}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Example;
    
    #[test]
    fn test1() {
        let x = Rc::new(Example);
        println!("引用计数：{}", Rc::strong_count(&x));
        let y = Rc::clone(&x);
        println!("引用计数：{}", Rc::strong_count(&x));
        println!("引用计数：{}", Rc::strong_count(&y));


        println!("A");
        drop(x);
        println!("引用计数：{}", Rc::strong_count(&y));
        println!("B");
        drop(y);
        println!("C");
    }

}