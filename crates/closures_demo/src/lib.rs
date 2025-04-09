use std::thread;

fn reference_demo() {
    let list = vec![1, 2, 3];
    // 1. 闭包定义前：list 未被借用
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); 
    // 闭包仅声明引用，未实际持有

    // 2. 闭包调用前：未实际产生借用
    println!("Before calling closure: {list:?}"); 

    only_borrows(); // 3. 调用时临时借用
    // 闭包执行完毕，借用释放

    // 4. 调用后：借用已释放
    println!("After calling closure: {list:?}");
}

fn mut_reference_demo() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");


    
    let mut brrows_muably = || list.push(4);
    //println!("After calling closure: {list:?}");
    brrows_muably();
    println!("After calling closure: {list:?}");
    
}

fn get_data_demo() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // 新建一个线程
    thread::spawn(move || println!("From thread: {list:?}"))
    .join().unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_rectangle() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut count = 0;
    
    
    list.sort_by_key(|r: &Rectangle| {
        count += 1;
        r.width;
    });
    println!("{list:#?}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_reference_demo() {
        reference_demo();
    }

    #[test]
    fn test_mut_reference_demo() {
        mut_reference_demo();
    }

    #[test]
    fn test_get_data_demo() {
        get_data_demo();
    }

    #[test]
    fn test_sort_rectangle() {
        sort_rectangle();
    }

}

