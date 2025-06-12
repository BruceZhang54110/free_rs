use std::{sync::{Arc, Mutex}, thread, time::Duration};


/// 演示死锁，a, b 两个线程，同时去抢锁，a抢lock1, b抢lock2, 之后再 a抢lock2, b抢lock1。
/// a线程必须等待b线程释放 lock2 锁，才能获取到lock2锁
/// b线程必须等待a线程释放 lock1 锁，才能获取到lock1锁
fn main() {
    // 创建两个互斥锁
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    // 克隆lock1
    let lock1_clone: Arc<Mutex<i32>> = Arc::clone(&lock1);
    // 克隆lock2
    let lock2_clone: Arc<Mutex<i32>> = Arc::clone(&lock2);

    let handler1 = thread::spawn(move || {
        println!("a 线程尝试获取 lock1...");
        let _guard = lock1.lock().unwrap();
        println!("a 线程 获取 lock1 成功...等待10 ms");
        thread::sleep(Duration::from_millis(10));

        println!("a 线程尝试获取 lock2...");
        let _guard2 = lock2_clone.lock().unwrap();
        println!("a 线程获取 lock2 成功...");
        println!("a 线程结束");
    });

    let handler2 = thread::spawn(move || {
        println!("b 线程尝试获取 lock2...");
        let _guard_2 = lock2.lock().unwrap();
        println!("b 线程 获取 lock2 成功...等待10 ms");
        thread::sleep(Duration::from_millis(10));

        println!("b 线程尝试获取 lock1...");
        let _guard2 = lock1_clone.lock().unwrap();
        println!("b 线程获取 lock2 成功...");
        println!("b 线程结束");
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
    println!("程序结束");

}