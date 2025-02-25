use reqwest::Client;
use serde_json::json;
use std::error::Error;

mod types_transaction;
mod types_block;

const NEAR_RPC_URL: &str = "https://archival-rpc.mainnet.near.org";

async fn print_transaction() -> Result<(), Box<dyn Error>> {
    let transaction_hash = "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U"; // Replace with your TX hash
    let sender_account_id = "miraclx.near"; // Replace with sender's account

    let client = Client::new();
    
    let payload = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "tx",
        "params": [transaction_hash, sender_account_id]
    });

    let response = client.post(NEAR_RPC_URL)
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let res = response.clone().get("result").unwrap().clone();

    let x: types_transaction::RpcTransactionResponse = serde_json::from_value(res)?;

    println!("{:#?}", x);

    Ok(())
}

async fn print_block() -> Result<(), Box<dyn Error>> {
    let block_id = "6ZWbbkgj2fpiaX3YVkBp4bYkPB1EgQH4P4LPfEzqVY6F"; // Replace with your TX hash

    let client = Client::new();
    
    let payload = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block",
        "params": [block_id]
    });

    let response = client.post(NEAR_RPC_URL)
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;


    let res = response.clone().get("result").unwrap().clone();

    let x: types_block::RpcBlockResponse = serde_json::from_value(res)?;

    println!("{:#?}", x);

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    if let Ok(x) = print_transaction().await {
        println!("hooray")
    } else {
        println!("error")
    }
    // println!("{}", serde_json::to_string_pretty(&response.clone())?);

    Ok(())
}