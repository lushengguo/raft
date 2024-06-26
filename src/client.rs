use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use regex::Regex;
use tokio::io::AsyncWriteExt;

// mod net;

async fn connect(address: &String) -> Result<TcpStream, tokio::io::Error> {
    match TcpStream::connect(address).await {
        Ok(socket) => {
            println!("Connected to server!");
            Ok(socket)
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            Err(e)
        }
    }
}

pub async fn start_client() {
    let address = "127.0.0.1:5000".to_string();
    match connect(&address).await // agent 
    {
        Ok(mut socket) => {
            let mut input = String::new();
            println!("please enter command like \"op key value\" and op should be \"Insert\", \"Modify\" or \"Delete\"");
            let re = Regex::new(r"(Insert|Modify|Delete) (\w+) (\w+)").unwrap();
            loop{
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        if re.is_match(input.as_str()) {
                            let input_len = input.len() as u64; 
                            let payload = input_len - 1;// without \n
                            let mut payload_buf = [0u8; 8];
                            payload_buf.copy_from_slice(&payload.to_be_bytes());
                            socket.write_all(&payload_buf).await.expect("failed to write data to socket");
 
                            let mut data_buf = vec![0u8; input_len as usize];
                            data_buf.copy_from_slice(input.as_bytes());
                            data_buf.pop();
                            socket.write_all(&data_buf).await.expect("failed to write data to socket");

                        } else {
                            println!("input is not valid, pattern should be operation key value ,and operation should be Insert, Modify or Delete.");
                        }
                    }
                    Err(error) => {
                        eprintln!("Error reading from stdin: {}", error);
                        return;
                    }
                }
            }
        },
        Err(error)=>{
            eprintln!("connect agent server failed: {}", error);
            return;
        }        
    }

}
