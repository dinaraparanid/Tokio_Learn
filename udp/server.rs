extern crate futures;
extern crate tokio;
extern crate tokio_stream;

use std::sync::Arc;
use tokio::{net::UdpSocket, sync::RwLock};

pub struct UdpFibonacciServer {
    mem: Vec<i128>,
}

impl UdpFibonacciServer {
    #[inline]
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server = Arc::new(RwLock::new(UdpFibonacciServer { mem: vec![0, 1] }));
        let socket = UdpSocket::bind("127.0.0.1:1337").await?;
        let mut buf = [0; 16];

        loop {
            let server = server.clone();

            let (_, addr) = socket.recv_from(&mut buf).await?;
            let index: usize = unsafe { std::mem::transmute_copy(&buf) };

            let number =
                tokio::task::spawn(async move { server.write().await.get_fibonacci(index) })
                    .await?;

            buf = unsafe { std::mem::transmute(number) };
            socket.send_to(&buf, addr).await?;
        }
    }

    #[inline]
    fn get_fibonacci(&mut self, index: usize) -> i128 {
        match self.mem.get(index) {
            Some(&number) => number,
            None => match index {
                0 => 0,
                1 => 1,
                _ => {
                    let number = self.get_fibonacci(index - 2) + self.get_fibonacci(index - 1);
                    self.mem.push(number);
                    number
                }
            },
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    UdpFibonacciServer::start().await
}
