//cargo add tokio --features rt --features rt-multi-thread --features macros --features net --features io-util
// cargo add tracing
//cargo add tracing-subscriber --features env-filter
//cargo run --example dredis
///powershell 下设置rust log的写法。
///$env:RUST_LOG="info";cargo run --example dredis
///简单的redis服务器
use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    //tcp lisener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis:listening on:{}", addr);
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from:{}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing conn with {}:{:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    warn!("connection {} closed.", raddr);
    Ok(())
}
