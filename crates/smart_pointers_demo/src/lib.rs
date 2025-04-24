mod rc_code;
mod refcell;
mod mess;
mod refcell_list;

use std::{ops::Deref, rc::Rc};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 关联类型

    fn deref(&self) -> &Self::Target { // 返回self这个不可变引用的目标，也就是T
        &self.0 // .0 取结构体的第一个元素
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct ConsumerSmartPointer {
    data: String,
}

impl Drop for ConsumerSmartPointer {

    fn drop(&mut self) {
        println!("Dropping ConsumerSmartPointer with data `{}`!", self.data);
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use List::*;
    use ListRc::*;


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

    #[test]
    fn test_box3() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

    }

    #[test]
    fn test_box4() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box5() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    #[test]
    fn test_drop() {
        let c = ConsumerSmartPointer {
            data: String::from("some data"),
        };
        let d = ConsumerSmartPointer {
            data: String::from("other data"),
        };
        println!("ConsumerSmartPointer created.");

    }

    #[test]
    fn test_rc1() {
        let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
        let b = List::Cons(3, Box::new(a));
        //let c = List::Cons(4, Box::new(a));
    }

    #[test]
    fn test_rc2() {
        let a = Rc::new(ConsRc(5, Rc::new(ListRc::ConsRc(10, Rc::new(NilRc)))));
        println!("a has {} strong references", Rc::strong_count(&a));
        let b = Rc::new(ConsRc(3, Rc::clone(&a)));
        {
            let c = Rc::new(ConsRc(4, Rc::clone(&a)));
            println!("a has {} strong references", Rc::strong_count(&a));

        }
        println!("a has {} strong references", Rc::strong_count(&a));
    }

    #[test]
    fn test_rc3() {
        let n = Rc::new(1);
        let m = Rc::clone(&n);
        // *m += 1; // 这里会报错，因为Rc<T>是不可变的
        println!("n = {}, m = {}", n, m);
    }

}