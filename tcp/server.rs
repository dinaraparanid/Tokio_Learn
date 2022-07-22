extern crate futures;
extern crate tokio;
extern crate tokio_stream;

use futures::StreamExt;
use std::sync::Arc;
use tokio_stream::wrappers::TcpListenerStream;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::RwLock,
};

pub struct TcpFibonacciServer {
    mem: Vec<i128>,
}

impl TcpFibonacciServer {
    #[inline]
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server = Arc::new(RwLock::new(TcpFibonacciServer { mem: vec![0, 1] }));
        let listener = TcpListenerStream::new(TcpListener::bind("127.0.0.1:1337").await?);

        listener
            .for_each(|stream| async {
                let server = server.clone();

                tokio::task::spawn(async move {
                    if let Ok(mut stream) = stream {
                        if let Ok(index) = stream.read_u64().await {
                            stream
                                .write_i128(
                                    tokio::task::spawn(async move {
                                        server.write().await.get_fibonacci(index as usize)
                                    })
                                    .await
                                    .unwrap_or(-1), // Error
                                )
                                .await
                                .unwrap_or_default();
                        }
                    }
                })
                .await
                .unwrap_or_default()
            })
            .await;

        Ok(())
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
    TcpFibonacciServer::start().await
}
