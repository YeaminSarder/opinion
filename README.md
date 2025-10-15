# to Run in localhost
``` bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cargo install basic-http-server
basic-http-server .
```
