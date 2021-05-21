build: metadata.wasm

metadata.wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/metadata.wasm ../build/metadata.wasm

.PHONY: clean
clean:
	rm ../build/metadata.wasm || true