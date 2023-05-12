use crate::raisin::*;
use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
use dotenv::dotenv;
use ethers::{
    abi::Abi,
    prelude::{k256::ecdsa::SigningKey, rand::thread_rng, Contract, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256},
    utils::parse_ether,
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
    GetRaisin(Index),
}

#[derive(Debug, Args)]
struct Init {
    #[clap(short, long)]
    amt: f32,
    #[clap(short, long)]
    token: String,
    #[clap(short, long)]
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
            println!("{:?}", new.address());
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
    let client = Arc::new(SignerMiddleware::new(provider, key_store));
    let contract = Contract::new(raisin.address, raisin.abi, Arc::clone(&client));

    match cli.command {
        Command::InitFund(x) => {
            let amount = parse_ether(x.amt)?;
            let token: Address = x.token.parse::<Address>()?;
            let receiver: Address = x.recipient.parse::<Address>()?;
            Raisin::init_fund(contract, amount, token, receiver).await?;
        }
        Command::Donate(x) => {
            let amount: U256 = parse_ether(x.amt)?;
            let token: Address = x.token.parse()?;
            let index: U256 = x.idx.into();
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            let token_contract = Contract::new(token, token_abi, Arc::clone(&client));
            approve_token(token_contract, raisin.address, amount).await?;
            Raisin::donate(contract, amount, token, index).await?;
        }
        Command::EndFund(x) => {
            let index: U256 = parse_ether(x.num)?;
            Raisin::end_fund(contract, index).await?;
        }
        Command::Withdraw(x) => {
            let index: U256 = parse_ether(x.num)?;
            Raisin::withdraw(contract, index).await?;
        }
        Command::Refund(x) => {
            let index: U256 = parse_ether(x.num)?;
            Raisin::refund(contract, index).await?;
        }
        Command::GetRaisin(x) => {
            let index: U256 = parse_ether(x.num)?;
            Raisin::get_raisin(contract, index).await?;
        }
        Command::BatchDonation(x) => {
            let amount: Vec<U256> = x.amt.iter().map(move |x| parse_ether(x).unwrap()).collect();
            let token: Vec<Address> = x.token.iter().map(move |x| x.parse().unwrap()).collect();
            let index: Vec<U256> = x.idx.iter().map(move |x| parse_ether(x).unwrap()).collect();
            let mut abi = std::fs::read_to_string("testtoken.json")?;
            abi = serde_json::from_str::<Value>(&abi)?.to_string();
            let token_abi: Abi = serde_json::from_str(&format!(r#"{}"#, abi))?;
            for i in 0..amount.len() {
                let token_contract =
                    Contract::new(token[i], token_abi.clone(), Arc::clone(&client));
                approve_token(token_contract, raisin.address, amount[i]).await?;
            }
            Raisin::batch_donate(contract, amount, token, index).await?;
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
