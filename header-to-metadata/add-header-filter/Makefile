build: add_header_filter.wasm

add_header_filter.wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/add_header_filter.wasm ../build/add_header_filter.wasm

.PHONY: clean
clean:
	rm ../build/add_header_filter.wasm || true