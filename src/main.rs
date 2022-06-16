// tokio frame
use bytes::Buf;
use bytes::BytesMut;
use mini_redis::{Frame, Result};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
            cursor: 0,
        }
    }
}

/// 读取帧
async fn read_frame(&mut self) -> Result<Option<Frame>> {
    loop {
        // 尝试从缓冲区的数据中解析出一个数据帧，
        // 只有当数据足够被解析时，才返回对应的帧
        if let Some(frame) = self.parse_frame()? {
            return Ok(Some(frame));
        }
        // 如果缓冲区中的数据还不足以被解析为一个数据帧，
        // 那么我们需要从 socket 中读取更多的数据
        //
        // 读取成功时，会返回读取到的字节数，0 代表着读到了数据流的末尾
        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            // 代码能执行到这里，说明了对端关闭了连接，
            // 需要看看缓冲区是否还有数据，若没有数据，说明所有数据成功被处理，
            // 若还有数据，说明对端在发送帧的过程中断开了连接，导致只发送了部分数据
            if self.buffer.is_empty() {
                return Ok(None);
            } else {
                return Err("connection reset by peer".into());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("rust main");
}
