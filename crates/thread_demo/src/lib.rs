use std::{sync::mpsc, thread, time::Duration};

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
            thread::sleep(Duration::from_millis(1000));
        }
    }).join().unwrap();
    
    println!("main thread...");
    for received in rx {
        println!("Got: {}", received);
    }
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

}