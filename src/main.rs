use rand::Rng;
use std::thread;
use std::{task, vec};
use tokio;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration, Instant};

fn main() {
    let mut rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .enable_time()
        .enable_io()
        .max_blocking_threads(7)
        .build()
        .unwrap();
    thread::sleep(Duration::from_millis(2000));
    let start = Instant::now();
    let mut handler = Vec::new();
    let h = rt.handle();
    for i in 0..=10000 {
        let handle = h.spawn(to(start));
        handler.push(handle);
        // println!("{:?}", handler)
    }
    let end = start.elapsed();
    thread::sleep(Duration::from_millis(5000));
    print!("{:?}", end);
}
async fn to(s: Instant) {
    let mut random = rand::thread_rng().gen_range(1000..2000);
    let task = sleep(Duration::from_millis(random)).await;
    // let task = thread::sleep(Duration::from_millis(random));
    println!("Time: {}, Time Taken: {:?}", random, s.elapsed());
    // println!("{:?}", s.elapsed());
}
