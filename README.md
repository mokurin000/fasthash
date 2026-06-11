# fasthash

Fast & lightweight sha2/sha3 series hasher, designed specially for large file hashing.

Up to 6x faster than `microsoft/uutils` on Windows,
with hardware acceleration support for aarch64/x86_64/loongarch64/wasm32 by default, riscv needs configuration[^1].

[^1]: https://docs.rs/sha2/latest/sha2/#backends

## Build

```bash
cargo +nightly build --release
```

## Run

```bash
cargo +nightly run --release -- --buf-size 32KiB sha3-256 file.vhd
```
