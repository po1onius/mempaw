use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut connect = TcpStream::connect("127.0.0.1:7389").await.unwrap();
    connect.write("SET a \"abc\"".as_bytes()).await.unwrap();
}
