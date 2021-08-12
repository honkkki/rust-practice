use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            // thread::spawn(move || {
            println!("number {} from spawn thread", i);
            thread::sleep(Duration::from_millis(1));
            // });
        }
    });

    for i in 1..6 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 阻塞等待handle执行完成
    handle.join().unwrap();
}
