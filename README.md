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

To use the keystore: `raisin -p <PATH_TO_FILE> [Command]`

Your keystore will be encrypted, make sure you remember your password! 

Command Options: 

`new-key <NAME>`,
  
`init-fund <AMOUNT> <TOKEN> <RECIPIENT>`,
  
`dontate <AMOUNT> <TOKEN> <INDEX>`,
  
`batch-donation <AMOUNTS> <TOKENS> <INDEX>`,
  
`end-fund <INDEX>`,
  
`withdraw <INDEX>`,
  
`refund <INDEX>`,
  
`get-raisin <INDEX>`
