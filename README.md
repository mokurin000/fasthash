# fasthash

Fast & lightweight sha2/sha3 series hasher.

## Download

Pre-release binaries are available on [Github Release](https://github.com/mokurin000/fasthash/releases/tag/nightly).

Notably, for glibc linux x86_64 target, only `glibc>=2.35` are supported, namely Ubuntu 22.04, or Debian 12.

## Hardware-Accelerated

Hardware acceleration support for aarch64/x86_64/loongarch64/wasm32 are runtime-detected by default, RISC-V requires additional configuration[^1].

[^1]: https://docs.rs/sha2/latest/sha2/#backends

## Benchmark

* File size: 66.9 GiB
* Storage: Predator GM7000
* OS: Windows 11 x86_64 22620
* CPU: Intel Core i7-12700H
* Hash algorithm: SHA-256

| Command                | Time  | Notes               |
| ---------------------- | ----- | ------------------- |
| `fasthash sha256 file` | 61s   |                     |
| `sha256sum file`       | 67s   | uutils 0.9.0        |
| hashlib*               | 68s   | Python 3.13.5       |
| `openssl sha256 file`  | 78s   | OpenSSL 3.6.0 MSVC  |
| `Get-FileHash file`    | 96.9s | PowerShell 5.1      |
| `sha256sum file`       | 123s  | Microsoft coreutils |

### Python script

```python
import sys, hashlib
print(hashlib.file_digest(open(sys.argv[1], "rb"), "sha256").hexdigest(), sys.argv[1])
```

## Limitation

Reading from stdin (`-`) is unsupported. It's heavily designed for large files.

## Build

```bash
cargo +nightly build --release
```

## Run

```bash
cargo +nightly run --release -- --buf-size 32KiB sha3-256 file.vhd
```
