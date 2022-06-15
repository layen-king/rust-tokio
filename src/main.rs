use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 先create
    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"some bytes").await?;
    println!("write a file {}", n);
    let mut f = File::open("foo.txt").await?;
    let mut buf = vec![];
    // 由于 buffer 的长度限制，当次的 `read` 调用最多可以从文件中读取 10 个字节的数据
    f.read_to_end(&mut buf).await?;
    println!("the bytes :{:?}", &buf);
    Ok(())
}
