use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, sol, sol_types::SolConstructor,
};
use koba::config::Deploy;
use dotenv::dotenv;

sol!(
    #[sol(rpc)]
    contract StylusOZErc20 {
        constructor(string memory name_, string memory symbol_);

        function name() external view returns (string name);
        function symbol() external view returns (string symbol);
    }
);

// Token properties
const TOKEN_NAME: &str = "StylusOZErc20";
const TOKEN_SYMBOL: &str = "SOZT";

#[tokio::main]
async fn main() {
    // Load dotenv variables
    dotenv().ok();
    let signer_private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set.");
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");

    let contract_address = deploy().await;
    
    // Instantiate signer wallet
    let signer = signer_private_key
        .parse::<PrivateKeySigner>()
        .expect("should parse the private key");
    let wallet = EthereumWallet::from(signer);

    // Instantiate provider
    let provider_rpc_url = rpc_url.parse().expect("should parse rpc url");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(provider_rpc_url);

    // Instantiate contract
    let contract = StylusOZErc20::new(contract_address, &provider);

    // Check token properties
    let call_result = contract.name().call().await.unwrap();
    assert_eq!(call_result.name, TOKEN_NAME.to_owned());

    let call_result = contract.symbol().call().await.unwrap();
    assert_eq!(call_result.symbol, TOKEN_SYMBOL.to_owned());
}

/// Deploy a `StylusOZErc20` contract to `RPC_URL` using `koba`.
async fn deploy() -> Address {
    // Load dotenv variables
    dotenv().ok();
    let signer_private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set.");
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");

    let args = StylusOZErc20::constructorCall {
        name_: TOKEN_NAME.to_owned(),
        symbol_: TOKEN_SYMBOL.to_owned(),
    };
    let args = alloy::hex::encode(args.abi_encode());

    let manifest_dir =
        std::env::current_dir().expect("should get current dir from env");

    // NOTE: It's expected that you compiled your contract beforehand.
    //
    // You should run `cargo build --release --target wasm32-unknown-unknown` to
    // get a wasm binary at `target/wasm32-unknown-unknown/release/{name}.wasm`.
    let wasm_path = manifest_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("stylus_erc20_oz_token.wasm");
    let sol_path = manifest_dir
        .join("token")
        .join("src")
        .join("constructor.sol");

    let config = Deploy {
        generate_config: koba::config::Generate {
            wasm: wasm_path.clone(),
            sol: sol_path,
            args: Some(args),
            legacy: false,
        },
        auth: koba::config::PrivateKey {
            private_key_path: None,
            private_key: Some(signer_private_key.to_owned()),
            keystore_path: None,
            keystore_password_path: None,
        },
        endpoint: rpc_url.to_owned(),
        deploy_only: false,
    };

    koba::deploy(&config).await.expect("should deploy contract")
}
