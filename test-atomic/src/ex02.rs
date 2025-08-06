use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Instant;

// for 次数
const N_TIMES: u64 = 123456;
// 线程数
const N_THREADS: usize = 10;
// 原子变量
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

// 原子类型的一个常用场景，就是作为全局变量来使用:
pub fn main() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    let mut thread_id = 0;
    for thread in threads {
        thread.join().unwrap();
        thread_id += 1;
        println!("thread_id: {}", thread_id);
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("Instant::now().sub(s) = {:?}", Instant::now().sub(s));
    println!("R = {}", R.load(Ordering::Relaxed));
}
