use tokio::net::TcpListener;
use tokio::net::TcpStream;

async fn connect(address: &String) -> Result<TcpStream, tokio::io::Error> {
    match TcpStream::connect(address).await {
        Ok(socket) => Ok(socket),
        Err(e) => Err(e),
    }
}


pub async send_to_node(message:&Vec<u8>, index:usize) -> Result<(), tokio::io::Error> {
}