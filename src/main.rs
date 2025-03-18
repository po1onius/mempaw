mod config;
mod memdata;
mod parse;
mod server;
mod utils;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.run().await;
}
