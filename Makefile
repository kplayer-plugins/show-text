OBJS= src/lib.rs

build: $(OBJS)
	cargo build --target wasm32-unknown-unknown --release
