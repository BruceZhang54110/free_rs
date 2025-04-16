enum List {
    Cons(i32, Box<List>),
    Nil,
}


#[cfg(test)]
mod tests {

    use super::*;
    use List::*;


    #[test]
    fn test_box() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn test_box2() {
        let mut n = 1;
        let b = Box::new(& mut n);
        **b += 1;
        println!("n = {}", n);
        
    }

}