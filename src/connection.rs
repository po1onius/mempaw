use std::{
    collections::HashMap,
    sync::{
        atomic::AtomicU32,
        mpsc::{Receiver, Sender, channel},
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, tcp::OwnedWriteHalf},
};

use crate::config::get_config;

static ID: AtomicU32 = AtomicU32::new(0);
fn get_id() -> u32 {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

struct OpMsg {
    cmd: String,
    conn_id: u32,
}

pub struct Connection {
    connsr: HashMap<u32, OwnedWriteHalf>,
    cmd_msg_tx: Sender<OpMsg>,
    cmd_msg_rx: Receiver<OpMsg>,
}

impl Connection {
    pub fn new() -> Self {
        let (cmd_msg_tx, cmd_msg_rx) = channel();
        Self {
            connsr: HashMap::new(),
            cmd_msg_tx,
            cmd_msg_rx,
        }
    }

    pub async fn run(&mut self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", get_config().port))
            .await
            .unwrap();

        loop {
            let (conn, _) = listener.accept().await.unwrap();
            let (mut conn_r, conn_w) = conn.into_split();
            let cmd_msg_tx_clone = self.cmd_msg_tx.clone();
            let conn_id = get_id();
            self.connsr.insert(conn_id, conn_w);
            if true {
                break;
            }
            tokio::spawn(async move {
                let mut buf = Vec::<u8>::new();
                let _ = conn_r.read(buf.as_mut_slice()).await;
                let _ = cmd_msg_tx_clone.send(OpMsg {
                    cmd: String::from_utf8(buf).unwrap(),
                    conn_id,
                });
            });
        }
    }

    pub async fn response(&mut self, conn_id: u32, content: &str) {
        self.connsr
            .get_mut(&conn_id)
            .unwrap()
            .write(content.as_bytes())
            .await;
    }

    pub fn fetch_cmd(&mut self) -> OpMsg {
        let op = self.cmd_msg_rx.recv().unwrap();
        op
    }
}
