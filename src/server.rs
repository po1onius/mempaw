use std::sync::Arc;
use std::thread;

use nom::AsBytes;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, channel};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, tcp::OwnedWriteHalf},
};

use crate::config::get_config;
use crate::memdata::CoreData;
use crate::parse::{
    frame::{Frame, read_frame},
    op::{Op, parse_str_set},
};
use crate::utils::get_id;

pub struct Server {
    data: CoreData,
}

impl Server {
    pub fn new() -> Self {
        Self {
            data: CoreData::default(),
        }
    }

    pub async fn run(&mut self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", get_config().port))
            .await
            .unwrap();

        let (op_tx, mut op_rx) = channel(100);

        thread::spawn(move || {
            loop {
                let op = op_rx.blocking_recv().unwrap();
            }
        });

        loop {
            let (conn, _) = listener.accept().await.unwrap();
            let op_tx_clone = op_tx.clone();
            if true {
                break;
            }
            tokio::spawn(async move {
                let mut handler = Handler {
                    connection: conn,
                    op_sender: op_tx_clone,
                };
                handler.run().await;
            });
        }
    }
}

struct Handler {
    connection: TcpStream,
    op_sender: Sender<Op>,
}

impl Handler {
    async fn run(&mut self) {
        let mut buf = [0; 1024];
        loop {
            let frame = read_frame(&mut self.connection).await.unwrap();
        }
    }
}
