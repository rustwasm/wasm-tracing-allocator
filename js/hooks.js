(function () {
  class Allocation {
    constructor(size, align, pointer) {
      this.size = size;
      this.align = align;
      this.pointer = pointer;
      this.stack = getStack();
    }
  }

  class InvalidFree {
    constructor(size, align, pointer) {
      this.size = size;
      this.align = align;
      this.pointer = pointer;
      this.stack = getStack();
    }
  }

  const liveAllocs = new Map();
  const invalidFrees = [];

  function getStack() {
    return Error()
      .stack
      .split("\n")
      .filter(frame => frame.match(/hooks\.js/) === null)
      .join("\n");
  }

  function onAlloc(size, align, pointer) {
    liveAllocs.set(pointer, new Allocation(size, align, pointer));
  }

  function onDealloc(size, align, pointer) {
    const wasLive = liveAllocs.delete(pointer);
    if (!wasLive) {
      invalidFrees.push(new InvalidFree(size, align, pointer));
    }
  }

  function onAllocZeroed(size, align, pointer) {
    onAlloc(size, align, pointer);
  }

  function onRealloc(
    oldPointer,
    newPointer,
    oldSize,
    newSize,
    align,
  ) {
    onDealloc(oldSize, align, oldPointer);
    onAlloc(newSize, align, newPointer);
  }

  function dumpTable(entries, { keyLabel, valueLabel, getKey, getValue }) {
    const byKey = new Map;
    let total = 0;

    for (const entry of entries) {
      const key = getKey(entry);
      const keyValue = byKey.get(key) || 0;
      const entryValue = getValue(entry);
      total += entryValue;
      byKey.set(key, keyValue + entryValue);
    }

    const table = [...byKey]
          .sort((a, b) => b[1] - a[1])
          .map(a => ({ [keyLabel]: a[0], [valueLabel]: a[1] }));

    table.unshift({ [keyLabel]: "<total>", [valueLabel]: total });

    console.table(table, [keyLabel, valueLabel]);
  }

  function getGlobal() {
    if (typeof self !== 'undefined') { return self; }
    if (typeof window !== 'undefined') { return window; }
    if (typeof global !== 'undefined') { return global; }
    throw new Error('unable to locate global object');
  }

  getGlobal().WasmTracingAllocator = {
    on_alloc: onAlloc,
    on_dealloc: onDealloc,
    on_alloc_zeroed: onAllocZeroed,
    on_realloc: onRealloc,

    dumpLiveAllocations(opts) {
      dumpTable(liveAllocs.values(), Object.assign({
        keyLabel: "Live Allocations",
        valueLabel: "Size (Bytes)",
        getKey: entry => entry.stack,
        getValue: _entry => 1,
      }, opts));
    },

    dumpInvalidFrees(opts) {
      dumpTable(invalidFrees, Object.assign({
        keyLabel: "Invalid Free",
        valueLabel: "Count",
        getKey: entry => entry.stack,
        getValue: _entry => 1,
      }, opts));
    },
  };
}());
