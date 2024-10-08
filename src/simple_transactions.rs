use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};

use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()>{
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();//spawn a ganache instance
    println!("HTTP Endpoint: {}", ganache.endpoint());//print the endpoint

    //get the first wallet managed by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();//wallet
    let first_address = wallet.address();
    println!(
        "wallet first address: {}",
        first_address.encode_hex::<String>()
        );

    //Provider is an Ethereum JsonRPC client
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    //balance of our account
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet first address balance:{}", first_balance);

    //balance of other account
    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!("Balance for address {}: {}",
    other_address_hex, other_balance);

//transaction for transfer 1000 wei to "other_address"

let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from (first_address);
//Send transaction and wait for receipt
let receipt = provider
        .send_transaction(tx, None)
        .await?
        .confirmations(1) // number of confirmations required
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("can not get block number")?
    );
    println!(
        "Balance of {} {}",
        other_address_hex,
        provider.get_balance(other_address, None).await?
    );
    Ok(())
}