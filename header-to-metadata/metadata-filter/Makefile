build: metadata_filter.wasm

metadata_filter.wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/metadata_filter.wasm ../build/metadata_filter.wasm

.PHONY: clean
clean:
	rm ../build/metadata_filter.wasm || true