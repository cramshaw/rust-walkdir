# Rust/WASM walk dir

Turns out that there isn't support for Walkdir in WASM yet.

This is a rough template for writing code to export as WASM for use in JS.

## Building

```
cargo build
```

or

```
wasm-pack build --target nodejs
```

## Running

```
node

const walker = require('./pkg/readfiles);

walker.findfiles();
```
