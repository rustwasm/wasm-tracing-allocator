# Contributing to `wasm-tracing-allocator`

Hi! We'd love to have your contributions! If you want help or mentorship, reach
out to us in a GitHub issue, or ping `fitzgen` in [`#wg-wasm` on the Rust
Programming Language Discord server](https://discordapp.com/invite/rust-lang)
and introduce yourself.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Code of Conduct](#code-of-conduct)
- [Building and Testing](#building-and-testing)
  - [Prerequisites](#prerequisites)
  - [Building](#building)
  - [Updating the `README.md`](#updating-the-readmemd)
  - [Testing](#testing)
- [Automatic Code Formatting](#automatic-code-formatting)
- [Contributions We Want](#contributions-we-want)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Code of Conduct

We abide by the [Rust Code of Conduct][coc] and ask that you do as well.

[coc]: https://www.rust-lang.org/en-US/conduct.html

## Building and Testing

### Prerequisites

Ensure that you have `cargo-readme` installed:

```
$ cargo install cargo-readme
```

### Building

```
cargo build
```

### Updating the `README.md`

```
cargo readme > README.md
```

### Testing

```
cargo test
```

## Automatic Code Formatting

We use [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) to enforce a
consistent code style across the whole code base.

You can install the latest version of `rustfmt` with this command:

```
rustup component add rustfmt
```

Once that is taken care of, you can (re)format all code by running this command
from the root of the repository:

```
cargo fmt --all
```

## Contributions We Want

* **Bug fixes!** Include a regression test if possible.

* **New analyses of allocations and deallocations!** File an issue before hand
  outlining the new analysis so we can all get on the same page and make sure it
  is the most awesome version of the analysis it can be.

* **Performance improvements!** The tracing has a lot of overhead, and if there
  are relatively easy ways to reduce that overhead, we'd like to do that.
