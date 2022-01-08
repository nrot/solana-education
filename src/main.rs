use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use std::path::{Path, PathBuf};

mod utils;

fn main() {
    println!("Start example");
    
    let client = RpcClient::new("http://localhost:8899".into());

    match utils::get_u8_64(Path::new("wallets/wallet1.json")) {
        Ok(d)=>{
            match Keypair::from_bytes(d.as_slice()){
                Ok(kp)=>{
                    client.get_balance()
                },
                Err(e)=>{
                    panic!("Error by data to keypair: {:?}", e);
                }
            }
        },
        Err(e)=>{
            panic!("Error by read data: {:?}", e);
        }
    }
    
}
