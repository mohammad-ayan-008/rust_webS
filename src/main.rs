mod server;
mod ResponseHandler;
use std::{fs::read_to_string};

use server::HttpServer;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    println!("Up and Running ");

    let mut result = HttpServer::run(8081).await.unwrap();
    result
        .listen(|data| match data {
            Some("/home") => String::from("<h1>home</h1>"),
            Some("/test") => read_to_string("src/res/test.html").unwrap(),
            _ => String::from("Error"),
        })
        .await;
}
