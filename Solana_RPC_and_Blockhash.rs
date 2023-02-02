use solana_client::rpc_client::RpcClient;

fn main() {
    let url = "https://api.solana.com";
    let client = match RpcClient::new(url) {
        Ok(client) => client,
        Err(error) => {
            println!("Error connecting to Solana RPC: {:?}", error);
            return;
        }
    };

    let blockhash = match client.get_recent_blockhash() {
        Ok(blockhash) => blockhash,
        Err(error) => {
            println!("Error retrieving blockhash: {:?}", error);
            return;
        }
    };

    println!("Blockhash: {:?}", blockhash);
}
