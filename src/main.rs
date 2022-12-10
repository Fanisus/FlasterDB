// Imports //
use rand::Rng;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time;
use tokio;
// MAIN //
// #[tokio::main]
fn main() {
    let (MainTx, ThreadRx) = mpsc::channel();
    let (ThreadTx, MainRx) = mpsc::channel();

    threads::create_thread(threads::ThreadType::Reader, ThreadTx, ThreadRx);

    let x: String = MainRx.recv().unwrap();
    println!("Data from MAin {}", x);
}

mod threads {
    use std::sync::mpsc;
    use std::thread;
    use std::time;

    pub enum ThreadType {
        Reader,
        Writer,
        Connection,
    }
    pub fn create_thread(
        thread_type: ThreadType,
        ThreadTx: mpsc::Sender<String>,
        ThreadRx: mpsc::Receiver<String>,
    ) {
        match thread_type {
            ThreadType::Reader => {
                thread::spawn(|| {
                    thread::sleep(time::Duration::from_millis(1000));
                    let val = String::from("hi");
                    println!("Thread {:?}", thread::current().id().to_owned());
                    ThreadTx.send(val).unwrap();
                });
            }
            _ => {}
        }
    }
}
