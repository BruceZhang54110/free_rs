use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("Tome"),
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
            String::from("hi1"),
            String::from("Tome1"),
            String::from("from1"),
            String::from("the1"),
            String::from("thread1")
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