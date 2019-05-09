# `wasm-tracing-allocator`


## `wasm-tracing-allocator`

TODO

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
static GLOBAL_ALLOCATOR: WasmTracingAllocator<System> = WasmTracingAllocator {
    inner: System
};
```

Now the `.wasm` binaries created via regular `cargo build`s and `wasm-pack
build`s will have their allocations and deallocations traced.

### Analyzing and Debugging

TODO

#### Memory Leaks

TODO

#### Use After Frees

TODO

