use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val={}", val); // 错误
    });

    let received = rx.recv().unwrap();
    println!("收到:{}", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..5 {
            let val = String::from("hi");
            tx.send(val).unwrap();
        }
    });

    for received in rx {
        println!("受到中：{}", received);
    }

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    
    thread::spawn(move || {
        for i in 1..5 {
            let val = String::from("jimo");
            tx.send(val).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 1..5 {
            let val = String::from("hehe");
            tx1.send(val).unwrap();
        }
    });

    for received in rx {
        println!("再次收到中：{}", received);
    }
}
