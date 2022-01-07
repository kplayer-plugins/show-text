.PHONY: clean
OBJS= src/lib.rs

build: $(OBJS)
	cargo build --target wasm32-wasi --release
	cp -f target/wasm32-wasi/release/show_text.wasm target/wasm32-wasi/release/show_text.kpe

clean:
	rm -rf target