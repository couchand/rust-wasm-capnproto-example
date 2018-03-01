.PHONY: all
all: rust js

.PHONY: rust
rust: dist/capnproto_example.wasm

.PHONY: js
js: dist/bundle.js

dist/capnproto_example.wasm: src/lib.rs schema/example.capnp
	mkdir site
	cargo +nightly wasm build --release
	mv site/capnproto_example.wasm dist
	rmdir site

dist/bundle.js: src/index.js src/io.js src/message.js build/schema/example.capnp.js
	yarn webpack --mode=production

build/schema/example.capnp.js: schema/example.capnp build
	capnpc -o node_modules/.bin/capnpc-js:build schema/example.capnp

build:
	mkdir build

clean:
	rm -rf build dist/bundle.js dist/bundle.js.map dist/capnproto_example.wasm target
