// Imports //
use crossbeam;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use rand::Rng;
use std::fs;
use std::io;
use std::thread;
use std::time;
use tokio;
// MAIN //
// #[tokio::main]
fn main() {
    let (tx, rx) = crossbeam::channel::unbounded();
    let tx_thread: Sender<threads::Message> = tx.clone();
    let rx_thread: Receiver<threads::Message> = rx.clone();
    let rxc = rx.clone();

    threads::create_thread(threads::ThreadType::Reader, tx_thread, rx_thread);
    listen(rxc)
}
fn listen(rx: Receiver<threads::Message>) {
    let x = rx.recv().unwrap();
    let y = x.data;
    println!("Data from MAin {}", y);
    listen(rx);
}
mod threads {
    use crossbeam;
    use std::sync::mpsc;
    use std::thread;
    use std::thread::ThreadId;
    use std::time;

    pub enum ThreadType {
        Reader,
        Writer,
        Connection,
    }
    pub struct Message {
        thread_id: ThreadId,
        query: String,
        data: Data
    }
    pub enum Data {
        String(String),
        Integer(i128)
    }
    fn get_type_of<T>(_: &T) -> String {
        std::any::type_name::<T>().to_string()
    }
    pub fn create_thread(
        thread_type: ThreadType,
        ThreadTx: crossbeam::channel::Sender<Message>,
        ThreadRx: crossbeam::channel::Receiver<Message>,
    ) {
        match thread_type {
            ThreadType::Reader => {
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_millis(1000));
                    let result = String::from("hi");
                    println!("Thread {:?}", thread::current().id().to_owned());
                    match get_type_of(&result).as_str() {
                        "String" => {
                            let data = Message {
                                thread_id: thread::current().id().to_owned(),
                                data: Data::String(result),
                                query: "hi".to_string()
                            };
                            ThreadTx.send(data).unwrap();
                        },

                    }
                    for i in 0..10 {
                    }
                });
            }
            _ => {}
        }
    }
}
