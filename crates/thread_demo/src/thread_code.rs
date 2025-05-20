#[cfg(test)]
mod tests {
    use std::{sync::{Arc, Mutex}, thread, time::Duration};


    #[test]
    fn test_thread() {

        let join_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawn thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        join_handle.join().unwrap();
        println!("Thread finished execution");
    }

    #[test]
    fn test_thread_move() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector:{v:?}");
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_print_n() {
        let mut n = 1;
        let t = thread::spawn(move || {
            n = n + 1;
            thread::spawn(move || {
                n = n + 1;
                println!("{n}")
            })
        });
        n = n + 1;
        t.join().unwrap().join().unwrap();
        println!("{n}");
        println!("-----------------------------------------------------------------");

    }

    #[test]
    fn test_multiple_threads() {
        // 创建一个共享的计数器
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        // 创建10个线程
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // 获取锁并增加计数器
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("线程 {:?} 完成工作", thread::current().id());
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 打印最终结果
        println!("最终计数: {}", *counter.lock().unwrap());
    }
}