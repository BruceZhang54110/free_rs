use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::refcell_list::List::{Cons, Nil};

    /// RefCell 搭配 Rc 实现对相同数据有用多个所有者，且可以对数据进行修改
    #[test]
    fn test1() {

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after= {:?}", b);
        println!("c after= {:?}", c);

    }

}