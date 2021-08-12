use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
       let data = String::from("hello");
        s.send(data).unwrap();
        thread::sleep(Duration::from_millis(1000));
        let s1 = s.clone();
        thread::spawn(move || {
            let data = String::from("rust");
            s.send(data).unwrap();
        });

        thread::sleep(Duration::from_millis(1000));
        let v = vec!["data", "from", "vector"];
        for str in v {
            s1.send(str.to_string()).unwrap();
        }
    });


    // let recv = r.recv().unwrap();
    // println!("got data: {}", recv);

    for rec in r {
        println!("got data: {}", rec);
        thread::sleep(Duration::from_millis(500));
    }
}
