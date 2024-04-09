// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel(); // 创建一个 mpsc 通道
    let queue = Queue::new();
    let queue_length = queue.length;

    let tx1 = tx.clone(); // 复制发送端
    let first_half_thread = thread::spawn(move || {
        for val in &queue.first_half {
            tx1.send(*val).unwrap(); // 发送值到通道
            thread::sleep(Duration::from_secs(1)); // 模拟延迟
        }
    });

    let tx2 = tx; // 使用原始发送端
    let second_half_thread = thread::spawn(move || {
        for val in &queue.second_half {
            tx2.send(*val).unwrap(); // 发送值到通道
            thread::sleep(Duration::from_secs(1)); // 模拟延迟
        }
    });

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    first_half_thread.join().unwrap();
    second_half_thread.join().unwrap();

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
