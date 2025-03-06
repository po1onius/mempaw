use std::sync::Arc;
use std::thread;

use tokio::sync::Mutex;

use crate::connection::Connection;
use crate::memdata::CoreData;
use crate::parse::{
    Op::{GET, SET},
    parse_str_set,
};

pub struct Server {
    data: CoreData,
    conns: Arc<Mutex<Connection>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            data: CoreData::default(),
            conns: Arc::new(Mutex::new(Connection::new())),
        }
    }

    pub fn run(&mut self) {
        let conns_clone1 = self.conns.clone();
        let conns_clone2 = self.conns.clone();

        thread::spawn(move || {
            loop {
                let op = conns_clone1.blocking_lock().fetch_op().op;
                match parse_str_set(&op) {
                    _ => (),
                }
            }
        });

        tokio::spawn(async move {
            conns_clone2.lock().await.run().await;
        });
    }
}
