install:
	cargo update && cargo bin --install

wasm.build:
	cargo wasmpack build --target bundler