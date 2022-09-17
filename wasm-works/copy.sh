echo "Copy wasm_singleton"
cp ./wasm-singleton/target/wasm32-unknown-unknown/release/wasm_singleton.wasm ./envoy-wasm
echo "Copy wasm-filter01"
cp ./wasm-filter01/target/wasm32-unknown-unknown/release/wasm_filter01.wasm ./envoy-wasm