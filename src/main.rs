#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::thread;
use sysinfo::{CpuExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use tokio::{fs, io, runtime, sync, time};
fn main() {
    // let rt = runtime::Builder::new_multi_thread()
    //     .enable_io()
    //     .enable_time()
    //     .worker_threads(128)
    //     .build()
    //     .unwrap();
    // let mut count = 0;
    // let mut handler = Vec::new();
    // let h = rt.handle();
    // for _i in 0..1000 {
    //     let handle = h.spawn(async move {
    //         time::sleep(time::Duration::from_secs(1)).await;
    //     });
    //     handler.push(handle);
    // }
    let mut sys = sysinfo::System::new();
    loop {
        sys.refresh_all();
        // RAM and swap information:
        println!(
            "Memory: {:.2}GB/{:.2}GB",
            sys.used_memory() as f32 / 1073741824 as f32,
            sys.total_memory() as f32 / 1073741824 as f32
        );
        // Number of CPUs:
        for cpu in sys.cpus() {
            print!("{:.2}% ", cpu.cpu_usage());
        }
        println!("\n");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
