use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("线程里的i={},v={:?}", i, v);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..5 {
        println!("main里的i={}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();
}
