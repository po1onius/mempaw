pub mod op {
    use nom::{
        IResult, Parser,
        bytes::complete::{escaped, tag, take_till1},
        character::complete::{char, none_of, space1},
        sequence::delimited,
    };

    #[derive(Debug)]
    pub enum Op {
        GET(String),
        SET(String, String),
        HSET(String, Vec<(String, String)>),
        HGET(String, String),
    }

    fn parse_simple_string(input: &str) -> IResult<&str, String> {
        let (input, s) = take_till1(|s| s == ' ')(input)?;
        Ok((input, s.to_string()))
    }

    fn parse_quoted_string(input: &str) -> IResult<&str, String> {
        let (input, s) = delimited(
            char('"'),
            escaped(none_of("\\\""), '\\', char('"')),
            char('"'),
        )
        .parse(input)?;
        Ok((input, s.to_string()))
    }

    pub fn parse_str_set(input: &str) -> IResult<&str, Op> {
        let (input, _) = tag("SET")(input)?;
        let (input, _) = space1(input)?;
        let (input, k) = parse_simple_string(input)?;
        let (input, _) = space1(input)?;
        let (input, v) = parse_quoted_string(input)?;
        Ok((input, Op::SET(k.to_string(), v.to_string())))
    }

    pub fn parse_hash_set(input: &str) -> IResult<&str, Op> {
        let (input, _) = tag("HSET")(input)?;
        let (input, _) = space1(input)?;
        let (mut input, k) = parse_simple_string(input)?;
        let mut pairs = Vec::new();
        loop {
            let (ls, _) = space1(input)?;
            let (lk, ink) = parse_simple_string(ls)?;
            let (ls, _) = space1(lk)?;
            let (lv, inv) = parse_quoted_string(ls)?;
            input = lv;
            pairs.push((ink, inv));
            if input == "" {
                return Ok((input, Op::HSET(k, pairs)));
            }
        }
    }

    pub fn parse_str_get(input: &str) -> IResult<&str, Op> {
        let (input, _) = tag("GET")(input)?;
        let (input, _) = space1(input)?;
        let (input, k) = parse_simple_string(input)?;
        Ok((input, Op::GET(k)))
    }

    pub fn parse_hash_get(input: &str) -> IResult<&str, Op> {
        let (input, _) = tag("HGET")(input)?;
        let (input, _) = space1(input)?;
        let (input, k) = parse_simple_string(input)?;
        let (input, _) = space1(input)?;
        let (input, ink) = parse_simple_string(input)?;
        Ok((input, Op::HGET(k, ink)))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let (void, r) = parse_hash_set(r#"HSET a b "c" d "e""#).unwrap();
            assert_eq!("", void);
            if let Op::HSET(k, v) = r {
                assert_eq!(2, v.len());
            }
        }
    }
}

pub mod frame {
    use std::iter::zip;

    use tokio::sync::mpsc::{Receiver, Sender, channel};

    use tokio::{io::AsyncReadExt, net::TcpStream};

    use crate::parse::op::{parse_hash_get, parse_str_set};

    use super::op::{Op, parse_hash_set, parse_str_get};

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
                let size = self.stream.read(&mut buf).await.unwrap();
                if size == 0 {
                    break;
                }
                for (i, j) in zip(buf[..buf.len() - 1].iter().enumerate(), buf[1..].iter()) {
                    if *i.1 == '\r' as u8 && *j == '\n' as u8 {
                        let op = String::from_utf8_lossy(&buf).to_string();
                        if let Some(op) = str2op(&op) {
                            self._cache.send(op).await;
                        }
                    }
                }
            }
        }

        pub async fn fetch(&mut self) -> Op {
            self.cache.recv().await.unwrap()
        }
    }

    fn str2op(s: &str) -> Option<Op> {
        if let Ok((s, op)) = parse_hash_get(s) {
            if s == "" {
                return Some(op);
            }
        }
        if let Ok((s, op)) = parse_hash_get(s) {
            if s == "" {
                return Some(op);
            }
        }
        if let Ok((s, op)) = parse_str_get(s) {
            if s == "" {
                return Some(op);
            }
        }
        if let Ok((s, op)) = parse_str_set(s) {
            if s == "" {
                return Some(op);
            }
        }
        None
    }
}
