<div align="center">

  <h1><code>wasm-tracing-allocator</code></h1>

  <strong>A global allocator for Wasm that traces allocations and deallocations for debugging purposes.</strong>

  <p>
    <a href="https://crates.io/crates/wasm-tracing-allocator"><img src="https://img.shields.io/crates/v/wasm-tracing-allocator.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/wasm-tracing-allocator"><img src="https://img.shields.io/crates/d/wasm-tracing-allocator.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/wasm-tracing-allocator"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://docs.rs/wasm-tracing-allocator">API Docs</a>
    <span> | </span>
    <a href="https://github.com/rustwasm/wasm-tracing-allocator/blob/master/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>


### About

`wasm-tracing-allocator` enables you to better debug and analyze memory leaks
and invalid frees in an environment where we don't have access to the
conventional tools like Valgrind. The tracing hooks are safely implemented in
JS, outside the Wasm module and its linear memory, to ensure that the tracing
code doesn't perturb results.

### Table of Contents

* [Enabling the Tracing Allocator](#enabling-the-tracing-allocator)
* [Analyzing and Debugging](#analyzing-and-debugging)

### Enabling the Tracing Allocator

First, add `wasm-tracing-allocator` to your `Cargo.toml`'s dependency list:

```toml
[dependencies]
wasm-tracing-allocator = "0.1.0"
```

Next, configure `wasm_tracing_allocator::WasmTracingAllocator` as the global
allocator:

```rust
// src/lib.rs

use std::alloc::System;
use wasm_tracing_allocator::WasmTracingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: WasmTracingAllocator<System> = WasmTracingAllocator(System);
```

Finally, make the JS implementations of the tracing hooks are available for your
Wasm module to import:

* On the Web, add this script *before* your Wasm module is instantiated:

  ```html
  <script src="https://unpkg.com/wasm-tracing-allocator@0.1.1/js/hooks.js"></script>
  ```

* On Node.js, require the hooks *before* your Wasm module is instantiated:

  ```js
  require("wasm-tracing-allocator");
  ```

### Analyzing and Debugging

Use your developer tools console to invoke methods of the global
`WasmTracingAllocator` object to get analyses about allocations and
deallocations.

The output is typically rendered with `console.table`:

[![Example output](https://raw.githubusercontent.com/rustwasm/wasm-tracing-allocator/master/live-allocations-dump.png)](https://raw.githubusercontent.com/rustwasm/wasm-tracing-allocator/master/live-allocations-dump.png)

#### `WasmTracingAllocator.dumpLiveAllocations`

Dump a table of live allocations to the console.

```js
WasmTracingAllocator.dumpLiveAllocations({
  keyLabel: String,
  valueLabel: String,
  getKey: Object => any,
  getValue: Object => Number,
});
```

* `keyLabel`: Optional. The string label used to describe the keys column in the
  table.

* `valueLabel`: Optional. The string label used to describe the values column in
  the table.

* `getKey`: Optional. Function from an allocation entry object to anything. The
  table will group and aggregate entries by their keys. Defaults to the stack at
  the time of the allocation.

* `getValue`: Optional. Function from an allocation entry object to a
  number. The values for all entries with the same key are summed. Defaults to
  the byte size of each allocation; a potential alternative would be to ignore
  the argument and return `1` to count the number of allocations instead.

#### `WasmTracingAllocator.dumpInvalidFrees`

Dump a table of invalid frees (double frees, frees of things that were never
allocated, etc...) to the console.

```js
WasmTracingAllocator.dumpInvalidFrees({
  keyLabel: String,
  valueLabel: String,
  getKey: Object => any,
  getValue: Object => Number,
});
```

* `keyLabel`: Optional. The string label used to describe the keys column in the
  table.

* `valueLabel`: Optional. The string label used to describe the values column in
  the table.

* `getKey`: Optional. Function from an invalid free entry object to anything. The
  table will group and aggregate entries by their keys. Defaults to the stack at
  the time of the deallocation.

* `getValue`: Optional. Function from an invalid free entry object to a
  number. The values for all entries with the same key are summed. Defaults to
  counting the number of invalid frees.

