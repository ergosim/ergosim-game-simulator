# Ergosim :: game :: simulator :: WASM

## Research

- [Rust Wasm (the book)]((https://rustwasm.github.io/book)

## Crates

### JS-sys [[crates.io](https://crates.io/crates/js-sys)]

Bindings for all JS global objects and functions in all JS environments like Node.js and browsers, built on 
`#[wasm_bindgen]` using the `wasm-bindgen` crate.

### Web-sys [[crates.io](https://crates.io/crates/web-sys)]

The web-sys crate provides raw wasm-bindgen imports for all of the Web's APIs. This includes:

- window.fetch
- Node.prototype.appendChild
- WebGL
- WebAudio

It does not include the JavaScript APIs that are guaranteed to exist in all standards-compliant ECMAScript environments, such as Array, Date, and eval. Bindings for these APIs can be found in the js-sys crate.