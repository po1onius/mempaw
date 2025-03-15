mod config;
mod memdata;
mod parse;
mod server;
mod utils;

use server::Server;

fn main() {
    let mut server = Server::new();
    server.run();
}
