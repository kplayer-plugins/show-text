OBJS=target/wasm32-unknown-unknown/release/show_text.wasm

build: $(OBJS)
	cargo build --target wasm32-unknown-unknown --release