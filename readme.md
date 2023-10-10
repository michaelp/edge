cargo build --target wasm32-wasi --release
wasmedge --reactor target/wasm32-wasi/release/edge.wasm add 2 2