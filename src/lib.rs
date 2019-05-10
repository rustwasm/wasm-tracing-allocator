/*!

**A global allocator for Wasm that traces allocations and deallocations for
debugging purposes.**

[![](https://docs.rs/wasm-tracing-allocator/badge.svg)](https://docs.rs/wasm-tracing-allocator/)
[![](https://img.shields.io/crates/v/wasm-tracing-allocator.svg)](https://crates.io/crates/wasm-tracing-allocator)
[![](https://img.shields.io/crates/d/wasm-tracing-allocator.svg)](https://crates.io/crates/wasm-tracing-allocator)
[![Build Status](https://dev.azure.com/rustwasm/wasm-tracing-allocator/_apis/build/status/rustwasm.wasm-tracing-allocator?branchName=master)](https://dev.azure.com/rustwasm/wasm-tracing-allocator/_build/latest?definitionId=2&branchName=master)

`wasm-tracing-allocator` enables you to better debug and analyze memory leaks
and invalid frees in an environment where we don't have access to the
conventional tools like Valgrind. The tracing hooks are safely implemented in
JS, outside the Wasm module and its linear memory, to ensure that the tracing
code doesn't perturb results.

## Table of Contents

* [Enabling the Tracing Allocator](#enabling-the-tracing-allocator)
* [Analyzing and Debugging](#analyzing-and-debugging)

## Enabling the Tracing Allocator

First, add `wasm-tracing-allocator` to your `Cargo.toml`'s dependency list:

```toml
[dependencies]
wasm-tracing-allocator = "0.1.0"
```

Next, configure `wasm_tracing_allocator::WasmTracingAllocator` as the global
allocator:

```no_run
// src/lib.rs
# fn main() {}

use std::alloc::System;
use wasm_tracing_allocator::WasmTracingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: WasmTracingAllocator<System> = WasmTracingAllocator(System);
```

Finally, make the JS implementations of the tracing hooks are available for your
Wasm module to import:

* On the Web, add this script *before* your Wasm module is instantiated:

  ```html
  <script src="https://unpkg.com/wasm-tracing-allocator@0.1.0/js/hooks.js"></script>
  ```

* On Node.js, require the hooks *before* your Wasm module is instantiated:

  ```js
  require("wasm-tracing-allocator");
  ```

## Analyzing and Debugging

Use your developer tools console to invoke methods of the global
`WasmTracingAllocator` object to get analyses about allocations and
deallocations.

The output is typically rendered with `console.table`:

[![Example output](https://raw.githubusercontent.com/rustwasm/wasm-tracing-allocator/master/live-allocations-dump.png)](https://raw.githubusercontent.com/rustwasm/wasm-tracing-allocator/master/live-allocations-dump.png)

### `WasmTracingAllocator.dumpLiveAllocations`

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

### `WasmTracingAllocator.dumpInvalidFrees`

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

 */

#![deny(missing_docs, missing_debug_implementations)]

use std::alloc::{GlobalAlloc, Layout};

#[doc(hidden)]
pub mod hooks;

/// A global allocator that traces the Wasm module's allocations and
/// deallocations.
///
/// It wraps some global allocator `A` that actually implements the allocation
/// and deallocation, and inserts its tracing after each invocation.
///
/// ## Example
///
/// Just give it the global allocator `A` to wrap, and add the
/// `#[global_allocator]` attribute. The module level documentation has an
/// example of wrapping the default system allocator. Here is an example of
/// wrapping [`wee_alloc`](https://github.com/rustwasm/wee_alloc):
///
/// ```ignore
/// // src/lib.rs
/// # fn main() {}
///
/// use wasm_tracing_allocator::WasmTracingAllocator;
/// use wee_alloc::WeeAlloc;
///
/// #[global_allocator]
/// static GLOBAL_ALLOCATOR: WasmTracingAllocator<WeeAlloc> =
///     WasmTracingAllocator(WeeAlloc::INIT);
/// ```
#[derive(Debug)]
pub struct WasmTracingAllocator<A>(pub A)
where
    A: GlobalAlloc;

unsafe impl<A> GlobalAlloc for WasmTracingAllocator<A>
where
    A: GlobalAlloc,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let pointer = self.0.alloc(layout);
        hooks::on_alloc(size, align, pointer);
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        self.0.dealloc(pointer, layout);
        hooks::on_dealloc(size, align, pointer);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let pointer = self.0.alloc_zeroed(layout);
        hooks::on_alloc_zeroed(size, align, pointer);
        pointer
    }

    unsafe fn realloc(&self, old_pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        let align = layout.align();
        let new_pointer = self.0.realloc(old_pointer, layout, new_size);
        hooks::on_realloc(old_pointer, new_pointer, old_size, new_size, align);
        new_pointer
    }
}
