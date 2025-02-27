use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, mpsc::Sender},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::config::get_config;

static ID: AtomicU32 = AtomicU32::new(0);
fn get_id() -> u32 {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

struct CmdMsg {
    cmd: String,
    conn_id: u32,
}

struct Connection {
    conns: HashMap<u32, TcpStream>,
    cmd_msg_tx: Sender<CmdMsg>,
}

impl Connection {
    pub async fn run(&mut self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", get_config().port))
            .await
            .unwrap();

        loop {
            let new_conn = listener.accept().await.unwrap();
            let cmd_msg_tx_clone = self.cmd_msg_tx.clone();
            self.conns.insert(get_id(), new_conn.0);
            if true {
                break;
            }

            tokio::spawn(async move { new_conn.0.read(buf) });
        }
    }
}
