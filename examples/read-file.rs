use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    read_file_async("foo.txt").await?;
    Ok(())
}

async fn read_file_async(filename: &str) -> io::Result<()> {
    let file = File::open(filename).await?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    // loop {
    //     if let Some(line) = lines.next_line().await? {
    //         print!("{}\r\n", line);
    //     } else {
    //         break;
    //     }
    // }

    while let Some(line) = lines.next_line().await? {
        print!("{}\r\n", line);
    }
    Ok(())
}
