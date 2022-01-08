use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{keypair::read_keypair_file},
    signer::Signer,
};

use std::path::{Path};

fn to_lamports(sol: u64)->u64{
    sol * 1000000000
}

fn main() {
    println!("Start example");

    let client = RpcClient::new("https://api.devnet.solana.com".into());
    let wal1 = read_keypair_file(Path::new("wallets/wallet1.json")).expect("Can`t read kp wallet 1 file");
    println!("Wallet 1 balance: {:?}", client.get_balance(&wal1.pubkey()));

    let wal2 = read_keypair_file(Path::new("wallets/wallet2.json")).expect("Can`t read kp wallet 2 file");
    println!("Wallet 2 balance: {:?}", client.get_balance(&wal2.pubkey()));

    println!("Airdrop to wal 1: {:?}", client.request_airdrop(&wal1.pubkey(), to_lamports(1)));
    println!("Wallet 1 balance: {:?}", client.get_balance(&wal1.pubkey()));
    
}
