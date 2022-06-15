use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("mini_redis is listening!");
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        // 使用多线程
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
/// 处理函数
async fn process(socket: TcpStream, db: Db) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    // let mut connect = Connection::new(socket);
    // if let Some(frame) = connect.read_frame().await.unwrap() {
    //     println!("GOT :{:?}",frame);
    //     let response = Frame::Error("unimplemented".to_string());
    //     connect.write_frame(&response).await.unwrap();
    // }
    use mini_redis::Command::{self, Get, Set};

    // `mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
    let mut connection = Connection::new(socket);
    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 当竞争不多的时候，使用阻塞性的锁去保护共享数据是一个正确的选择。
                // 当一个锁竞争触发后，当前正在执行任务(请求锁)的线程会被阻塞，并等待锁被前一个使用者释放。
                // 这里的关键就是：锁竞争不仅仅会导致当前的任务被阻塞，还会导致执行任务的线程被阻塞，
                // 因此该线程准备执行的其它任务也会因此被阻塞！

                // 默认情况下，Tokio 调度器使用了多线程模式，此时如果有大量的任务都需要访问同一个锁，
                // 那么锁竞争将变得激烈起来。当然，你也可以使用 current_thread 运行时设置
                // ，在该设置下会使用一个单线程的调度器(执行器)，所有的任务都会创建并执行在当前线程上，因此不再会有锁竞争。
                // 值被存储为 `Vec<u8>` 的形式
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // 将请求响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
}
