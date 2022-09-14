dfx canister create query
dfx build query
ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/query.wasm -o ./target/wasm32-unknown-unknown/release/query.wasm
# TODO it doesn't seem like ic-wasm shrink does anything
# ic-wasm ./target/wasm32-unknown-unknown/release/query.wasm -o ./target/wasm32-unknown-unknown/release/query.wasm shrink
gzip -f -k ./target/wasm32-unknown-unknown/release/query.wasm
# TODO possibly add the cargo test stuff back for candid
dfx canister install --mode upgrade --wasm target/wasm32-unknown-unknown/release/query.wasm.gz query
