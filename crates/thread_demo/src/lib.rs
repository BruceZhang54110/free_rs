use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn move_value() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

}

fn channel_value() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Got: {}", received);
}

fn channel_multiple_value() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    println!("main thread...");
    for received in rx {
        println!("Got: {}", received);
    }
}

fn channel_clone_multiple_value() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread")
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    println!("main thread...");
    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_lock() {
    let m: Mutex<i32> = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m= {:?}", m);
}

fn multiple_thread_mutex_lock() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle: thread::JoinHandle<()> = thread::spawn(move || {

            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 主线程会获取锁，并打印出程序的结果
    println!("Result: {}", *counter.lock().unwrap());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_value() {
        move_value();
    }

    #[test]
    fn test_channel_value() {
        channel_value();
    }

    #[test]
    fn test_channel_multiple_value() {
        channel_multiple_value();
    }

    #[test]
    fn test_channel_clone_multiple_value() {
        channel_clone_multiple_value();
    }

    #[test]
    fn test_mutex_lock() {
        mutex_lock();
    }

    #[test]
    fn test_multiple_thread_mutex_lock() {
        multiple_thread_mutex_lock();
    }

}