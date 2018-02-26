.PHONY: all
all: rust js

.PHONY: rust
rust: site/capnproto_example.wasm

.PHONY: js
js: site/bundle.js

site/capnproto_example.wasm: src/lib.rs schema/example.capnp
	cargo +nightly wasm build --release

site/bundle.js: js/index.js js/io.js js/message.js build/schema/example.capnp.js
	yarn webpack

build/schema/example.capnp.js: schema/example.capnp build
	yarn capnpc

build:
	mkdir build

clean:
	rm -rf build site/bundle.js site/bundle.js.map site/capnproto_example.wasm target
