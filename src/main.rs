use tokio::net::{TcpStream, TcpListener};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("mini_redis is listening!");
    loop{
        let (socket,_) = listener.accept().await.unwrap();
        // 使用多线程
        tokio::spawn(async move  {
            process(socket).await;
        });
    }
}

async fn process(socket:TcpStream){
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut connect = Connection::new(socket);
    if let Some(frame) = connect.read_frame().await.unwrap() {
        println!("GOT :{:?}",frame);
        let response = Frame::Error("unimplemented".to_string());
        connect.write_frame(&response).await.unwrap();
    }
}