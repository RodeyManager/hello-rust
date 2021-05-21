use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1、创建线程 与 使用 join 等待所有线程结束

    // let handle = thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("child threed number: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 0..5 {
    //     println!("main threed number: {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // 2、线程与 move 闭包

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("在子线程中，主线程中变量v： {:?}", v);
    });
    handle.join().unwrap();
    // 这里不能再使用 v了，所有权已经移动到handle线程中
    // println!("v: {:?}", v);

    // 3、使用消息传递在线程间传送数据

    // mpsc 是 多个生产者，单个消费者

    // 当发送者或接收者任一被丢弃时可以认为通道被 关闭（closed）了。
    /*  let (tx, rx) = mpsc::channel();
    let _handle = thread::spawn(move || {
        let message = String::from("子线程消息");
        tx.send(message).unwrap();
        // 一下代码编译失败，因为message所有权已经ts.send转移到接受方了
        // println!("message: {}", message);
    });
    let rev_message = rx.recv().unwrap();
    println!("主线程接受到信息：`{}`", rev_message); */

    // 发送多个值并观察接收者的等待
    let (tx, rx) = mpsc::channel();
    let tx_clone = mpsc::Sender::clone(&tx);

    let _handle = thread::spawn(move || {
        let ms = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for m in ms {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 通过克隆发送者来创建多个生产者  mpsc::Sender::clone(&tx)
    let _handle2 = thread::spawn(move || {
        let ms = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for m in ms {
            tx_clone.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 多次接受
    for rev_message in rx {
        println!("Got: {}", rev_message);
    }

    // 4、共享状态并发

    // 4.1、互斥器一次只允许一个线程访问数据
    // 以下为单线程互斥器例子 （Mutex<T>的 API）
    // Mutex<T> 必须在调用lock（获取锁）之后才能使用其值<T>
    let m = Mutex::new(2);
    {
        let mut number = m.lock().unwrap();
        *number = 6;
    }
    println!("m = {:?}", m);

    // 4.2、在线程间共享 Mutex<T> 和 原子引用计数 Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性

    // Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。
    // 使用 Mutex<T> 来改变 Arc<T> 中的内容

    // 5、使用 Sync 和 Send trait 的可扩展并发

    // *** Rust语言本身没有提供并发模型及相关基础设施，所以开发者可以自己编写并发模型或使用其他人提供的 crate
    // *** 但是Rust中有两个并发相关的概念：std::marker 中的 Send 和 Sync trait;
    // *** - 通过 Send 允许在线程间转移所有权
    // *** - Sync 允许多线程访问
}
