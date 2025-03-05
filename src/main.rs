mod config;
mod connection;
mod memdata;
mod parse;
mod server;

use server::Server;

fn main() {
    let mut server = Server::new();
    server.run();
}
