# Raisin-CLI
A command line tool to directly interact with Raisin! 
_______________________________________________________
## Installation

### 0. Install Rust

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 1. Clone this repository 

  `git clone https://github.com/crypdoughdoteth/Raisin-CLI/`

### 2. Create .env file with the following key:

`API_KEY`

API_KEY's corresponding value is your JSON-RPC provider's API key to connect to Ethereum,

### 3. Build it & run

Ensure that you are in the crate's root directory & in your terminal type `cargo install --path <PATH-TO-FOLDER>`

## Usage 

To invoke the Raisin-CLI use the command `raisin`! 

To use this tool, you need a keystore. Generate a new one by using: `raisin -p <PATH_TO_DIR> <NAME>`.

To use the tool: `raisin -p <PATH_TO_FILE> <Command>`

Your keystore will be encrypted, make sure you remember your password! 

NOTE: The testnet version of Raisin is currently on Sepolia!

Command Options: 

`new-key <NAME>`: Creates a new keystore,
  
`init-fund <AMOUNT> <TOKEN> <RECIPIENT>`: Create a Raisin,
  
`dontate <AMOUNT> <TOKEN> <INDEX>`: Donate to a Raisin,
  
`batch-donation <AMOUNTS> <TOKENS> <INDICES>`: Donate to multiple Raisins,
  
`end-fund <INDEX>`: End a Raisin you own,
  
`withdraw <INDEX>`: Withdraw from your successful Raisin,
  
`refund <INDEX>`: Refund from an unsuccessful Raisin,
  
`get-raisin <INDEX> <TOKEN>`: Get information about a Raisin,

`get-balance <ADDRESS> <TOKEN>`: Get balance of any token at some address,

`transfer-tkn <AMOUNT> <TOKEN> <RECIPIENT>`: Transfer any token from your keystore to an arbitrary address,

`transfer-eth <AMOUNT> <RECIPIENT>`: Transfer Ether (or native asset) from your keystore to an arbitrary address 
