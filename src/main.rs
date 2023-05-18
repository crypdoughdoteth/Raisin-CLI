use crate::raisin::*;
use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::{
    abi::Abi,
    prelude::{k256::ecdsa::SigningKey, rand::thread_rng, Contract, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, TransactionRequest},
    utils::{parse_units, parse_ether},
};
use serde_json::Value;
use std::sync::Arc;
mod raisin;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to keystore, generates a new one if not supplied
    #[clap(short, long)]
    path: String,
    /// Whatcha wanna do?
    #[command(subcommand)]
    command: Command,
}
#[derive(Debug, Args)]
struct Index {
    num: u32,
}

#[derive(Debug, Args)]
struct Name {
    name: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generates a new key store
    NewKey(Name),
    /// start a fund
    InitFund(Init),
    /// donate to a fund
    Donate(Donation),
    /// donate to multiple funds
    BatchDonation(Donations),
    /// end your fund
    EndFund(Index),
    /// withdraw from your fund (if successful)
    Withdraw(Index),
    /// refund from a fund (if !succesful)
    Refund(Index),
    /// get information on a Raisin
    GetRaisin(FetchRaisin),
    /// get token balance
    GetBalance(Balance),
    /// Transfer tokens
    TransferTkn(Init),
    /// Transfer Ether
    TransferEth(SendEth),
    /// Testnet Token Airdrop
    Test,
}
#[derive(Debug, Args)]
struct SendEth {
    amt: f32,
    to: String
}
#[derive(Debug, Args)]
struct Init {
    amt: f32,
    token: String,
    recipient: String,
}
#[derive(Debug, Args)]
struct Donation {
    amt: f32,
    token: String,
    idx: u32,
}

#[derive(Debug, Args)]
struct Donations {
    amt: Vec<f32>,
    token: Vec<String>,
    idx: Vec<u32>,
}

#[derive(Debug, Args)]
struct Balance {
    addy: String,
    token: String,
}
#[derive(Debug, Args)]
struct FetchRaisin {
    idx: u32,
    token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let provider = Provider::<Http>::try_from(api_key)?;
    let raisin = Raisin::new();
    let cli = Cli::parse();
    let key_store: Wallet<SigningKey> = match &cli.command {
        Command::NewKey(n) => {
            let (new, _) = LocalWallet::new_keystore(
                &cli.path,
                &mut thread_rng(),
                request_new_password(),
                Some(&n.name),
            )?;
            println!("Your new address is {:?}", new.address());
            return Ok(());
        }
        _ => {
            let password =
                rpassword::prompt_password("Please enter a password to decrypt this key: ")?;
            let wallet = LocalWallet::decrypt_keystore(&cli.path, password)?;
            println!("{:?}", &wallet.address());
            wallet.with_chain_id(provider.get_chainid().await.unwrap().as_u64())
        }
    };
    let addy = key_store.address();
    let client = Arc::new(SignerMiddleware::new(provider, key_store));
    let contract = Contract::new(raisin.address, raisin.abi, Arc::clone(&client));
    match cli.command {
        Command::InitFund(x) => {
            let token: Address = x.token.parse::<Address>()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            let decimals = get_decimals(token_contract.clone()).await? as usize;
            let amount = parse_units(x.amt, decimals)?;
            let receiver: Address = x.recipient.parse::<Address>()?;
            Raisin::init_fund(contract, amount.into(), token, receiver, decimals).await?;
            println!("Fund successfully initialized!");
        }
        Command::Donate(x) => {
            let token: Address = x.token.parse()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            let decimals = get_decimals(token_contract.clone()).await? as usize;
            let amount = parse_units(x.amt, decimals)?;
            let index: U256 = x.idx.into();
            approve_token(token_contract, raisin.address, amount.into(), decimals).await?;
            Raisin::donate(contract, amount.into(), token, index, decimals).await?;
            println!("Donation successful!");
        }
        Command::EndFund(x) => {
            let index: U256 = x.num.into();
            Raisin::end_fund(contract, index).await?;
            println!("Successfully ended fund!");
        }
        Command::Withdraw(x) => {
            let index: U256 = x.num.into();
            Raisin::withdraw(contract, index).await?;
            println!("Successfully withdrew funds!");
        }
        Command::Refund(x) => {
            let index: U256 = x.num.into();
            Raisin::refund(contract, index).await?;
            println!("Refund successful!");
        }
        Command::GetRaisin(x) => {
            let token: Address = x.token.parse()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            let decimals = get_decimals(token_contract.clone()).await? as usize;
            let index: U256 = x.idx.into();
            Raisin::get_raisin(contract, index, decimals).await?;
        }
        Command::BatchDonation(x) => {
            let amount: Vec<f32> = x.amt;
            let mut parsed_amounts: Vec<U256> = Vec::new();
            let token: Vec<Address> = x.token.iter().map(move |x| x.parse().unwrap()).collect();
            let index: Vec<U256> = x.idx.iter().map(move |x| U256::from(*x)).collect();
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            for i in 0..amount.len() {
                let token_contract =
                    Contract::new(token[i], token_abi.clone(), Arc::clone(&client));
                let decimals: usize = get_decimals(token_contract.clone()).await? as usize;
                approve_token(
                    token_contract,
                    raisin.address,
                    parse_units(amount[i], decimals)?.into(),
                    decimals,
                )
                .await?;
                parsed_amounts.push(parse_units(amount[i], decimals)?.into());
            }
            Raisin::batch_donate(contract, parsed_amounts, token, index).await?;
            println!("Batch of donations sent successfully!");
        }
        Command::GetBalance(x) => {
            let token: Address = x.token.parse()?;
            let addy: Address = x.addy.parse()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            let decimals = get_decimals(token_contract.clone()).await? as usize;
            get_balance(token_contract, addy, decimals).await?;
        }
        Command::TransferTkn(x) => {
            let token: Address = x.token.parse::<Address>()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            let decimals = get_decimals(token_contract.clone()).await? as usize;
            let amount = parse_units(x.amt, decimals)?;
            let receiver: Address = x.recipient.parse::<Address>()?;
            transfer(token_contract, token, amount.into(), receiver, decimals).await?;
            println!("Token Transfer successful!");
        },
        Command::TransferEth(x) => {
            println!("Sending {} ether to {} ...", &x.amt, &x.to);
            let tx = TransactionRequest::new()
                .to(x.to.parse::<Address>()?)
                .value(U256::from(parse_ether(x.amt)?))
                .from(addy);
            client.send_transaction(tx, None).await?.await?;
            println!("Ether transfer successful!");
        },
        Command::Test => {
            let tkn = "0x7A56e2F6e2965a3569Fe3BD9c8f65E565C0941ef".parse::<Address>()?;
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let contract = Contract::new(tkn, token_abi, Arc::clone(&client));
            let call = contract.method::<_, ()>("mint", ())?;
            let pending = call.send().await?;
            pending.confirmations(6).await?;
            println!("Successfully minted test tokens!")
        }
        _ => (),
    }

    Ok(())
}

pub(crate) fn request_new_password() -> String {
    let password =
        rpassword::prompt_password("Please enter a password to encrypt this private key: ")
            .unwrap();

    let confirmation = rpassword::prompt_password("Please confirm your password: ").unwrap();

    if password != confirmation {
        println!("Passwords do not match -- try again!");
        std::process::exit(1);
    }
    password
}
