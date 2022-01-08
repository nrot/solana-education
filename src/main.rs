use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{keypair::read_keypair_file},
    signer::Signer,
};

use std::path::{Path};

fn main() {
    println!("Start example");

    let client = RpcClient::new("http://localhost:8899".into());
    let kp = read_keypair_file(Path::new("wallets/wallet1.json")).expect("Can`t read kp file");
    let pb: Pubkey = kp.pubkey();
    let bl = client.get_balance(&pb);
    println!("Wallet 1 balance: {:?}", bl);
}
