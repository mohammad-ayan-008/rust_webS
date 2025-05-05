#![feature(async_fn_traits)]
use std::{error::Error, sync::Arc};

use server::HttpServer;
use tokio_postgres::NoTls;

// use server::HttpServer;
// use tokio_postgres::NoTls;
mod server;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Box<dyn Error>> {
    let (client,connection) = tokio_postgres::connect("postgresql://aion:1234@localhost:5432/testdb", NoTls)
        .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });
    let arc_client = Arc::new(client);

    let mut server = HttpServer::run(8081).await?;

    server
        .listen(async move |data| {
            let client = Arc::clone(&arc_client);
                match data.as_deref() {
                    Some("/db") => {
                        let name = "Ferris";
                        let email ="mohammadayanafaq@gmail.com";
                        // arc_client is moved into this async block
                        client
                            .execute(
                                "INSERT INTO person (name, email) VALUES ($1, $2)",
                                &[&name, &email],
                            )
                            .await; 
                    
                        "written".to_owned()
                    }
                    Some("/home") => "<h1>home</h1>".to_string(),
                    _ => "Error".to_string(),
                
            }
        })
        .await;

    Ok(())
}
