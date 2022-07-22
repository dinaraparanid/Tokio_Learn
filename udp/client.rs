extern crate tokio;

use tokio::net::UdpSocket;

#[inline]
async fn get_fibonacci(index: u64) -> Result<i128, Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:1338").await?;
    socket.connect("127.0.0.1:1337").await?;

    let mut buf: [u8; 16] = unsafe { std::mem::transmute_copy(&index) };
    socket.send(&buf).await?;

    buf = [0; 16];
    socket.recv(&mut buf).await?;

    Ok(unsafe { std::mem::transmute(buf) })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(
        for ind in 0..100 {
            println!("{}", get_fibonacci(ind).await?)
        }
    )
}
