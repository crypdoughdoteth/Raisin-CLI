use anyhow::Result;
use ethers::{
    abi::Abi,
    prelude::ContractInstance,
    providers::Middleware,
    types::{Address, U256},
    utils::format_ether,
};
use serde_json::Value;
use std::borrow::Borrow;

pub(crate) struct Raisin {
    pub abi: Abi,
    pub address: Address,
}

impl Raisin {
    pub(crate) fn new() -> Raisin {
        let mut abi = std::fs::read_to_string("abi.json").unwrap();
        abi = serde_json::from_str::<Value>(&abi).unwrap().to_string();
        Self {
            abi: serde_json::from_str(&format!(r#"{}"#, abi)).unwrap(),
            address: "0x7e37Cd627C75DB9b76331F484449E5d98D5C82c5"
                .parse::<Address>()
                .unwrap(),
        }
    }
    pub(crate) async fn init_fund<T: Clone + Borrow<M>, M: Middleware + 'static>(
        contract: ContractInstance<T, M>,
        amt: U256,
        tkn: Address,
        receiver: Address,
    ) -> Result<()> {
        println!("You have began a fund on Raisin with a goal of: Token Contract: {}, Amount: {}, for Cause #{}", &tkn, &amt, &receiver );
        let call =
            contract.method::<_, (U256, Address, Address)>("initFund", (amt, tkn, receiver))?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }

    pub(crate) async fn donate<T, M>(
        contract: ContractInstance<T, M>,
        amt: U256,
        tkn: Address,
        index: U256,
    ) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        println!(
            "Donation pending ... Token Contract: {}, Amount: {} to cause #{} ",
            &tkn,
            format_ether(amt.as_u32()),
            &index
        );
        let call = contract.method::<_, (Address, U256, U256)>("donateToken", (tkn, index, amt))?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }
    pub(crate) async fn batch_donate<T, M>(
        contract: ContractInstance<T, M>,
        amt: Vec<U256>,
        tkn: Vec<Address>,
        index: Vec<U256>,
    ) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        println!(
            "Donations pending ... \nTokens: {:?}, \nAmounts: {:?}, \nInices: {:?}",
            &tkn, &amt, &index
        );
        let call = contract.method::<_, (Vec<Address>, Vec<U256>, Vec<U256>)>(
            "batchTokenDonate",
            (tkn, index, amt),
        )?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }

    pub(crate) async fn end_fund<T, M>(contract: ContractInstance<T, M>, index: U256) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        println!("Ending fund... #{}", &index);
        let call = contract.method::<_, U256>("endFund", index)?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }
    pub(crate) async fn get_raisin<T, M>(
        contract: ContractInstance<T, M>,
        index: U256,
    ) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        let raisin_info = contract.method::<_, U256>("raisins", index)?.call().await?;
        println!("{}", raisin_info);
        Ok(())
    }
    pub(crate) async fn withdraw<T, M>(contract: ContractInstance<T, M>, index: U256) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        println!("Withdrawing from successful fundraiser ... #{}", &index);
        let call = contract.method::<_, U256>("fundWithdraw", index)?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }
    pub(crate) async fn refund<T, M>(contract: ContractInstance<T, M>, index: U256) -> Result<()>
    where
        T: Clone + Borrow<M>,
        M: Middleware + 'static,
    {
        println!("Refunding from unsuccessful fundraiser ... #{}", &index);
        let call = contract.method::<_, U256>("refund", index)?;
        let pending = call.send().await?;
        pending.confirmations(6).await?;
        Ok(())
    }
}

pub(crate) async fn approve_token<T, M>(
    contract: ContractInstance<T, M>,
    spender: Address,
    amt: U256,
) -> Result<()>
where
    T: Clone + Borrow<M>,
    M: Middleware + 'static,
{
    println!(
        "Approving for Raisin... Contract: {}, Amount: {}",
        &contract.address(),
        format_ether(amt.as_u32())
    );
    let call = contract.method::<_, (U256, Address)>("Approve", (spender, amt))?;
    let pending = call.send().await?;
    pending.confirmations(6).await?;
    Ok(())
}
