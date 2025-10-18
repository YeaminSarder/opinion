cargo build --package opinion --target wasm32-unknown-unknown
mkdir -p public
cp target/wasm32-unknown-unknown/debug/opinion.wasm public/opinion.wasm
