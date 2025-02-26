use std::{collections::HashMap, sync::mpsc::Sender};
use tokio::net::{TcpListener, TcpStream};

use crate::config::get_config;

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
            self.conns.insert(1, new_conn.0);
            if true {
                break;
            }

            tokio::spawn(async {});
        }
    }
}
