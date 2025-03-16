This repository shows how to link Rust and TypeScript through WASM.

It is necessary to make WASMs of rust-wasm beforehand.

```sh
cd rust_wasm
wasm-pack build --target nodejs --output-dir pkg-node
wasm-pack build --target deno --output-dir pkg-deno
wasm-pack build --target web --output-dir pkg-web
wasm-pack build --target bundler --output-dir pkg-bundler
```
