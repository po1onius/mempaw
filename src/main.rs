mod config;
mod connection;
mod memdata;
mod server;

use server::Server;

fn main() {
    let server = Server::new();
}
