use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(5);
    {
        let mut num = mutex.lock().unwrap();
        *num = 6;
    }
    println!("mutex ={mutex:?}");
}