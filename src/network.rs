use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use crate::blockchain::Blockchain;
use crate::crypto::hash_to_hex;
use serde_json::json;

pub fn start_server(bc: Blockchain, address: &str) {
    let listener = TcpListener::bind(address).expect("Could not bind address");
    println!("Node listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chain_clone = bc.clone();
                thread::spawn(move || handle_connection(stream, chain_clone));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, bc: Blockchain) {
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes]);

    println!("Received: {}", request);

    if request.contains("get_latest_block") {
        let latest = bc.chain.last().unwrap();
        let json = serde_json::to_string(&latest).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
    } else {
        stream.write_all(b"Unknown command").unwrap();
    }
}

use crate::block::Block;

pub fn request_latest_block(peer: &str) -> Option<Block> {
    match TcpStream::connect(peer) {
        Ok(mut stream) => {
            // Send the request
            if stream.write_all(b"get_latest_block").is_err() {
                return None;
            }

            // Read the response
            let mut buffer = [0; 2048];
            let len = stream.read(&mut buffer).ok()?;

            // Convert bytes to string and deserialize to Block
            let response = String::from_utf8_lossy(&buffer[..len]);
            serde_json::from_str::<Block>(&response).ok()
        }
        Err(_) => {
            println!("❌ Could not connect to peer {}", peer);
            None
        }
    }

    use crate::block::Block;

    pub type PeerList = Vec<String>;

    pub fn broadcast_block(peers: &PeerList, block: &Block) {
    let json = serde_json::to_string(block).unwrap();

    for peer in peers {
        if let Ok(mut stream) = TcpStream::connect(peer) {
            let _ = stream.write_all(b"send_block\n");
            let _ = stream.write_all(json.as_bytes());
            println!("Send block to {}", peer);
        } else {
            println!("Could not connect to {}", peer);
        }
    }
}

}