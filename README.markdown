rust-wasm-capnproto-example
===========================

This is a little demonstration of Rust compiling to WASM, and using
Cap'n Proto messages as a high-bandwidth communication channel between
the Rust/WASM code and JavaScript code.

dependencies
------------

Languages:

- Rust (currently a nightly)
- Your browser of choice supporting JavaScript and WASM

Cap'n Proto libraries:
- [capnproto-rust](https://github.com/capnproto/capnproto-rust)
- [capnp-ts](https://github.com/jdiaz5513/capnp-ts)

Build tools:
- Rustup & Cargo
- [cargo-wa](https://github.com/mgattozzi/cargo-wa)
- Yarn or NPM
- Webpack
- Make

getting started
---------------

1. Install the Rust nightly.

```sh
> rustup toolchain install nightly
```

2. Install & setup cargo-wa.

```sh
> cargo install cargo-wa
> cargo wa setup
```

3. Clone the repository.

```sh
> git clone git@github.com:couchand/rust-wasm-capnproto-example.git
```

4. Install JavaScript dependencies.

```sh
> yarn
```

5. Build the project.

```sh
> make
```

6. Pull it up in your browser.

```sh
> xdg-open site/index.html
```

more information
----------------

Read the code & build config, there's only like 200 lines of code.

* [killercup/wasm-experiments](https://github.com/killercup/wasm-experiments/)
* [Cap'n Proto](https://capnproto.org/)

caveat
------

I'm new to Rust, so the code's not great.  Please submit a PR!

##### ╭╮☲☲☲╭╮ #####
