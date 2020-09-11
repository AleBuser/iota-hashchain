# Private IOTA Hashchain
The Hashchain is a solution to create a chain of proofs about the integrity of data stored in a database. It is heavily inspired by the principles and mechanisms of traditional blockchains and it uses the IOTA-Tangle as a temporay immutable storage layer.
Full description at: https://medium.com/@alebuser  

NOTE: this is a prototype and unexpected behavior should be expected


# Running the Hashchain Prototype 

## Requirements
* [Rust](https://www.rust-lang.org/tools/install)
* [PostgreSQL](https://www.postgresql.org/download/linux/ubuntu/)
* [diesel_cli](https://crates.io/crates/diesel_cli)

## Getting Started
Get the Code and enter the directory:  
`git clone https://github.com/AleBuser/iota-hashchain && cd iota-hashchain`  

Change the username and password in the .env for your local postgres database:  
`nano .env`  

Edit the configuration file to set the target IOTA address onto which to write the Hashchain:  
`nano config.json`  
 
Initialize the database:  
`diesel setup`    
Run the migration to generate the tables:  
`diesel migration run`  

Generate the genesis block:  
`cargo run --release --bin genesis`

Start the Node with an API Key (in this case: example_api_key):  
`cargo run --release --bin hashchain example_api_key`    

The Node will print the base endpoint and start generationg blocks!  

You can now send data to the endpoint specified in the config file, for example with cURL:  
`curl --location --request POST '127.0.0.1:8585/add_to_mempool' 
--header 'x-api-key: example_api_key'
--header 'Content-Type: application/json'
--data-raw '{  
    "sensor_data": "Some test data"  
}'`  

You can now also request the content of the database to get the block history, for example with cURL:  
`curl --location --request GET '127.0.0.1:8585/get_history'`
  
(The block is stored on the tangle encoded in Base64, you can take the "Text" version of the signature message in the transaction from TheTangle.org and decode it using a tool like https://www.base64decode.org/)
 
## Integrity verification
To verify the integrity of the local Hashchain using the built in verifier:  
`cargo run --release --bin verify`
