build: envoy_rate_limit_header.wasm

envoy_rate_limit_header.wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/envoy_rate_limit_header.wasm ./build/envoy_rate_limit_header.wasm

.PHONY: clean
clean:
	rm ./build/envoy_rate_limit_header.wasm || true