use std::{thread::{self, JoinHandle}, time::Instant};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::ops::Sub;

// + 1 的次数
const N_TIMES: u64 = 10000;

// 线程数
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }

    })

}

fn main() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}",Instant::now().sub(s));


}