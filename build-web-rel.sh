cargo build --package opinion --target wasm32-unknown-unknown --profile wasm-release
mkdir -p public
cp target/wasm32-unknown-unknown/wasm-release/opinion.wasm public/opinion.wasm
wasm-opt -O --output public/opinion.wasm public/opinion.wasm 
wasm-bindgen --out-name  opinion --out-dir public --target web public/opinion.wasm
