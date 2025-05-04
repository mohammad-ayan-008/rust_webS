use std::{error::Error, sync::Arc};

use server::HttpServer;
use tokio_postgres::NoTls;

// use server::HttpServer;
// use tokio_postgres::NoTls;
mod server;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = tokio_postgres::connect("postgresql://aion:1234@localhost:5432/testdb", NoTls)
        .await?
        .0;
    let arc_client = Arc::new(client);

    let mut server = HttpServer::run(8081).await?;

    server
        .listen(move |data| {
            let client = Arc::clone(&arc_client);
            async  move{
                match data.as_deref() {
                    Some("/db") => {
                        let name = "Ferris";
                        let data_opt: Option<&[u8]> = None;
                        // arc_client is moved into this async block
                        client
                            .execute(
                                "INSERT INTO person (name, data) VALUES ($1, $2)",
                                &[&name, &data_opt],
                            )
                            .await
                            .unwrap();
                        "".to_owned()
                    }
                    Some("/home") => "<h1>home</h1>".to_string(),
                    _ => "Error".to_string(),
                }
            }
        })
        .await;

    Ok(())
}

//
//
// use std::{error::Error, sync::Arc};
// use tokio_postgres::NoTls;
// use server::HttpServer;
//
// #[tokio::main(flavor = "multi_thread", worker_threads = 16)]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // 1. Establish a single Client and wrap in Arc
//     let client = tokio_postgres::connect(
//         "postgresql://aion:1234@localhost:5432/testdb",
//         NoTls,
//     )
//     .await?
//     .0;
//     let arc_client = Arc::new(client);
//
//     // 2. Start the server
//     let mut server = HttpServer::run(8081).await?;
//
//     // 3. Pass in a closure that clones Arc for each request
//     server
//         .listen({
//          // capture by value
//             move |data: Option<String>| {
//                 // clone for this request
//                 let client_handle = Arc::clone(&arc_client);
//                 async move {
//                     match data.as_deref() {
//                         Some("/db") => {
//                             // use the cloned handle
//                             client_handle
//                                 .execute(
//                                     "INSERT INTO person (name, data) VALUES ($1, $2)",
//                                     &[&"Ferris", &Option::<&[u8]>::None],
//                                 )
//                                 .await
//                                 .unwrap();
//                             "".to_owned()
//                         }
//                         Some("/home") => "<h1>home</h1>".to_string(),
//                         _ => "Error".to_string(),
//                     }
//                 }
//             }
//         })
//         .await;
//
//     Ok(())
// }
//
