# rust-wasm-capnproto-example

This is a little demonstration of Rust compiling to WASM, and using
Cap'n Proto messages as a high-bandwidth communication channel between
the Rust/WASM code and JavaScript code.

## Uses

* Rust (currently a nightly)

* Cap'n Proto libraries:

  * [capnproto-rust](https://github.com/capnproto/capnproto-rust)
  * [capnp-ts](https://github.com/jdiaz5513/capnp-ts)

## build dependencies

* Rustup & Cargo
* [cargo-wasm](https://github.com/mgattozzi/cargo-wasm)
* [`capnp` tool](https://capnproto.org/install.html)
* Yarn or NPM
* Make

## runtime dependencies

* Your browser of choice supporting JavaScript and WASM

## getting started

1. Install the Rust nightly.

```sh
> rustup toolchain install nightly
```

2. Install & setup cargo-wasm.

```sh
> cargo install cargo-wasm
> cargo wasm setup
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
> yarn start
```

## more information

Read the code & build config, there's only like 200 lines of code.

* [killercup/wasm-experiments](https://github.com/killercup/wasm-experiments/)
* [Cap'n Proto](https://capnproto.org/)

## caveat

I'm new to Rust, so the code's not great. Please submit a PR!

##### ╭╮☲☲☲╭╮
