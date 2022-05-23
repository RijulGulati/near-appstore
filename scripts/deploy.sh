# Script to deploy contract to testnet

# Build code
echo "# Building contract code"
cargo build --manifest-path ../Cargo.toml --target wasm32-unknown-unknown --release

echo "\n DEPLOYING CONTRACT TO TESTNET \n"

# Deploy on testnet
near dev-deploy --wasmFile ../target/wasm32-unknown-unknown/release/appstore.wasm --initFunction new --initArgs '{}'