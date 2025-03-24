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
        Len,
        Data,
    }

    enum ParseArrayState {}

    // *k\r\n
    const CLI_LEN_MIN: u32 = 4;

    struct Parser<'a> {
        buf: &'a Vec<u8>,
        cursor: usize,
        len: usize,
        res: Vec<Op>,
        array_cache: Vec<CliValue>,
        array_state: i32,
    }

    impl Parser<'_> {
        fn parse(&mut self) -> usize {
            let mut cursor = 0;
            loop {
                if cursor >= self.buf.len() {
                    break;
                }
                match self.buf[cursor] {
                    b'*' => {
                        if self.array_state > 0 {
                            panic!("array in array?");
                        }
                        self.array_state = 0;
                        cursor += 1;
                    }
                    b'+' => {
                        let (unit_data, peek) =
                            parse_pair(self.buf.as_ref(), self.buf.len() - cursor).unwrap();
                    }
                    _ => {
                        panic!("");
                    }
                }
            }

            3
        }
    }

    fn parse_pair(buf: &[u8], size: usize) -> Result<(&[u8], usize), ()> {
        for (i, v) in buf.iter().enumerate() {
            if *v == b'\r' {
                let len = str::from_utf8(&buf[..i]).unwrap().parse::<usize>().unwrap();
                let last = i + 1 + len;
                if last >= size {
                    return Err(());
                }
                return Ok((&buf[(i + 1)..=last], last));
            }
        }
        Err(())
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
