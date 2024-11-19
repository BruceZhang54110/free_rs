use std::{thread, time::Duration};
use std::sync::mpsc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawn thread!", i);
    //         thread::sleep(Duration::from_micros(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_micros(1));
    // }
    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
