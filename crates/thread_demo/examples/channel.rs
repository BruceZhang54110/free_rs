use std::{sync::mpsc, thread, time::Duration};

fn main() {
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
            thread::sleep(Duration::from_secs(2));
        }
    });
    println!("main thread ...");
    let s: Result<String, mpsc::RecvError> = rx.recv();
    for recv in rx {
        println!("Got :{}", recv);
    }
        


}