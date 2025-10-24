cargo build --package opinion --target wasm32-unknown-unknown
mkdir -p public
cp target/wasm32-unknown-unknown/debug/opinion.wasm public/opinion.wasm
wasm-bindgen --out-name  opinion --out-dir public --target web public/opinion.wasm
