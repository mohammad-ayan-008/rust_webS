
use std::{error::Error, sync::Arc};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream},
};

pub struct HttpServer {
    tcp_listener: TcpListener,
}

impl HttpServer {
    pub async fn run(port: i32) -> Result<HttpServer, Box<dyn Error>> {
        let server = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(HttpServer {
            tcp_listener: server,
        })
    }

    pub async fn listen<F>(&mut self,closure: F)
    where
        F: AsyncFn(Option<String>) -> String+ Send + Sync + 'static,
        for<'a> F::CallRefFuture<'a>: Send
    {
        let  rc = Arc::new( closure);

        loop {
            if let Ok((mut i, _)) = self.tcp_listener.accept().await {
                let closure = Arc::clone(&rc);
                tokio::spawn(async move {
                    let data = Self::fetch_response(&mut i).await.unwrap();
                    let cp = closure(data).await;
                    Self::write_to_server(&mut i, cp).await
                });
            }
        }
    }

    pub async fn write_to_server(stream: &mut TcpStream, data: String) {
        let mut buff = BufWriter::new(stream);
        buff.write_all(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
                data.len()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
        buff.write_all(data.as_bytes()).await.unwrap();
        buff.flush().await.unwrap();
    }

    pub async fn fetch_response(
        tcp_stream: &mut TcpStream,
    ) -> Result<Option<String>, Box<dyn Error +Send>> {
        let mut buffer = tokio::io::BufReader::new(tcp_stream);
        let mut line = String::new();
        let _ = buffer.read_line(&mut line).await;

        if let Some(s) = line.split(" ").nth(1) {
            Ok(Some(s.to_string()))
        } else {
            Ok(Some("".to_string()))
        }
    }
}
