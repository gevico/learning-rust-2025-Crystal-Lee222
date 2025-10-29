// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



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

// 修改：返回两个 JoinHandle，让调用者等待
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    let tx1 = tx.clone(); // 克隆 sender 给第一个线程
    let tx2 = tx;         // 原始 tx 给第二个线程

    let handle1 = thread::spawn(move || {
        for val in &q.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in &q.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    (handle1, handle2) // 返回句柄，供等待
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 获取两个线程的句柄
    let (handle1, handle2) = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 等待两个线程完成（虽然 rx 已关闭，但确保资源释放）
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}