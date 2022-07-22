extern crate tokio;

use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

#[inline]
async fn get_fibonacci(index: u64) -> Result<i128, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:1337").await?;
    stream.write_u64(index).await.unwrap_or(());
    Ok(stream.read_i128().await?)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(
        for ind in 0..100 {
            println!("{}", get_fibonacci(ind).await?)
        }
    )
}
