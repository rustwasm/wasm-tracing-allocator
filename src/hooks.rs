use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Invoked after each `GlobalAlloc::alloc`.
    #[wasm_bindgen(js_namespace = WasmTracingAllocator)]
    pub fn on_alloc(size: usize, align: usize, pointer: *mut u8);

    /// Invoked after each `GlobalAlloc::dealloc`.
    #[wasm_bindgen(js_namespace = WasmTracingAllocator)]
    pub fn on_dealloc(size: usize, align: usize, pointer: *mut u8);

    /// Invoked after each `GlobalAlloc::alloc_zeroed`.
    #[wasm_bindgen(js_namespace = WasmTracingAllocator)]
    pub fn on_alloc_zeroed(size: usize, align: usize, pointer: *mut u8);

    /// Invoked after each `GlobalAlloc::realloc`.
    #[wasm_bindgen(js_namespace = WasmTracingAllocator)]
    pub fn on_realloc(
        old_pointer: *mut u8,
        new_pointer: *mut u8,
        old_size: usize,
        new_size: usize,
        align: usize,
    );
}
