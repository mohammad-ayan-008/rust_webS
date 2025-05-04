mod ResponseHandler;
mod server;
use std::{error::Error, fs::read_to_string};

use server::HttpServer;
use tokio_postgres::NoTls;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Up and Running ");
    let (client, connection) =
        tokio_postgres::connect("postgresql://testdb?ayan@localhost:5432", NoTls).await?;
    let mut result = HttpServer::run(8081).await?;
    result
        .listen(async move |data| match data {
            Some("/db") => {
                let name = "Ferris";
                let data = None::<&[u8]>;
                client.execute(
                    "INSERT INTO person (name, data) VALUES ($1, $2)",
                    &[&name, &data],
                ).await.unwrap();
                
                String::from("data send")
            }
            Some("/home") => String::from("<h1>home</h1>"),
            Some("/test") => read_to_string("src/res/test.html").unwrap(),
            _ => String::from("Error"),
        })
        .await;

    Ok(())
}
