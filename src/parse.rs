pub mod op {
    #[derive(Debug)]
    pub enum Op {
        GET(String),
        SET(String, String),
        HSET(String, Vec<(String, String)>),
        HGET(String, String),
    }

    pub enum CliValue {
        Str(String),
        Int(i32),
        Float(f32),
        Bin(Vec<u8>),
    }

    enum ParseState {
        Normal,
        Array(usize),
    }

    enum ParseArrayState {}

    // *k\r\n
    const CLI_LEN_MIN: u32 = 4;

    struct Parser<'a> {
        buf: &'a [u8; 1024],
        cursor: usize,
        len: usize,
        res: Vec<Op>,
        state: ParseState,
    }

    impl Parser<'_> {
        fn parse(&mut self) -> usize {
            match self.buf[self.cursor] {
                b'*' => {}
                _ => {}
            }

            3
        }

        fn parse_len(&mut self) -> usize {}
    }

    fn parse(buf: &[u8; 1024], len: usize) -> (Vec<CliValue>, usize) {
        match buf[0] {
            b'*' => {
                let idx = buf[1..].iter().position(|v| *v == b'\r').unwrap();
                let mut len_arry = str::from_utf8(&buf[1..idx])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                loop {
                    if len_arry == 0 {
                        break;
                    }

                    len_arry -= 1;
                }
            }
            b'+' => {}
            _ => {
                panic!("parse cli");
            }
        }

        (Vec::new(), 0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {}
    }
}

pub mod frame {
    use std::iter::zip;

    use tokio::sync::mpsc::{Receiver, Sender, channel};

    use tokio::{io::AsyncReadExt, net::TcpStream};

    use crate::parse::op::Op;

    pub struct Frame {
        pub op: String,
    }

    pub struct OpCache {
        stream: TcpStream,
        _cache: Sender<Op>,
        cache: Receiver<Op>,
    }

    impl OpCache {
        fn new(stream: TcpStream) -> Self {
            let (tx, rx) = channel(100);
            Self {
                stream,
                _cache: tx,
                cache: rx,
            }
        }

        pub async fn run(&mut self) {
            loop {
                let mut buf = [0; 1024];
                let mut offset = 0;
                let size = self.stream.read(&mut buf).await.unwrap();
                if size == 0 {
                    break;
                }
                for (i, j) in zip(buf[..buf.len() - 1].iter().enumerate(), buf[1..].iter()) {
                    if *i.1 == '\r' as u8 && *j == '\n' as u8 {
                        let op = String::from_utf8_lossy(&buf).to_string();
                        if let Some(op) = str2op(&op) {
                            self._cache.send(op).await;
                            offset = i.0;
                        }
                    }
                }
            }
        }

        pub async fn fetch(&mut self) -> Op {
            self.cache.recv().await.unwrap()
        }
    }
}
