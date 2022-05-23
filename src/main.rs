#![allow(dead_code)]

use infrastructure::web_server::WebServer;
// use infrastructure::command_line::CommandLine;

mod application;
mod domain;
mod infrastructure;
mod interface;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = WebServer::new();
    server.run().await
}


// #[tokio::main]
// async fn main() -> Result<()> {
//     let command_line = CommandLine::new().await?;
//     
//     if let Err(e) = command_line.start().await {
//         eprintln!("Error: {}", e);
//     }
//     Ok(())
// }