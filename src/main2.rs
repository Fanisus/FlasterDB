// Imports //
use crossbeam;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use rand::Rng;
use std::fs;
use std::io;
use std::thread;
use std::thread::ThreadId;
use std::time;
use tokio;

use crate::threads::Data;
// MAIN //
// #[tokio::main]
fn main() {
    let (tx, rx) = crossbeam::channel::unbounded();
    let tx_thread = tx.clone();
    let rx_thread = rx.clone();
    let rxc = rx_thread.clone();
    threads::create_thread(threads::ThreadType::Reader, tx_thread, rx_thread);
    listen(rxc)
}
fn setup() {
    tokio::spawn({
        
    })
}

fn listen(rx: Receiver<(String, String, String)>) {
    let (thread_name, data, query) = rx.recv().unwrap();
    println!("Data from MAin {:?}",thread_name);
    listen(rx);
}

mod threads {
    use crossbeam;
    use std::any::Any;
    use std::fmt;
    use std::sync::mpsc;
    use std::thread;
    use std::thread::ThreadId;
    use std::time;

    pub enum ThreadType {
        Reader,
        Writer,
        Connection,
    }
    #[derive(Debug)]
    pub struct Message {
        thread_id: ThreadId,
        query: String,
        data: Data,
    }
    #[derive(Debug)]
    pub enum Data {
        String(String),
        Integer(i128),
    }
    fn get_type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }
    pub fn create_thread(
        thread_type: ThreadType,
        thread_tx: crossbeam::channel::Sender<(String, String, String)>,
        thread_rx: crossbeam::channel::Receiver<(String, String, String)>,
    ) {
        match thread_type {
            ThreadType::Reader => {
                thread::Builder::new().name(format!("Reader {}", 1).to_string()).spawn(move || {
                    thread::sleep(time::Duration::from_millis(1000));
                    let result = String::from("hi");
                    println!("{}", get_type_of(&result));
                    let data = (
                        thread::current().name().unwrap().to_owned(),
                        String::from(result),
                        "hi".to_owned(),
                    );
                    thread_tx.send(data).unwrap();
                    println!("Sent");
                });
            }
            _ => {}
        }
    }
}
