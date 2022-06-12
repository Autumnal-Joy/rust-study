use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn concurrency() {
    {
        // thread::spawn 创建新线程
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    {
        // 消息传递
        {
            // thread::channel 创建消息传递通道
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
            });

            let received = rx.recv().unwrap();
            println!("Got: {}", received);
        }

        {
            // multiple producer, single consumer
            let (tx, rx) = mpsc::channel();

            let tx1 = tx.clone();
            thread::spawn(move || {
                let vals = vec![
                    String::from("1"),
                    String::from("11"),
                    String::from("111"),
                    String::from("1111"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            });

            thread::spawn(move || {
                let vals = vec![
                    String::from("2"),
                    String::from("22"),
                    String::from("222"),
                    String::from("2222"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }
    }

    {
        // 共享状态
        {
            // lock 获得锁, drop 释放锁
            let m = Mutex::new(5);
            {
                let mut num = m.lock().unwrap();
                *num = 6;
            }
            dbg!(m);
        }

        {
            // Arc 实现线程间资源计数
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
        }
    }
}
