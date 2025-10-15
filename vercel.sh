cargo build --package opinion --target wasm32-unknown-unknown --release
mkdir -p public
cp target/wasm32-unknown-unknown/release/opinion.wasm public/opinion.wasm
cp index.html public/index.html
cp gl.js public/gl.js
#cargo build --package opinion_api --bin api-hello --target "x86_64-unknown-linux-musl"
