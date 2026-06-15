# fasthash

Fast & lightweight sha2/sha3 series hasher.

## Download

Pre-release binaries are available on [GitHub Releases](https://github.com/mokurin000/fasthash/releases/tag/nightly).

Notably, for glibc Linux targets, only `glibc >= 2.31` is supported, namely since:

- Debian 11
- Ubuntu 20.04
- Fedora 32
- RHEL/Rocky Linux/AlmaLinux 9
- openSUSE Leap 15.3
- SUSE Linux Enterprise Server 15 SP3
- Slackware 15

## Hardware-Accelerated

Hardware acceleration support for aarch64/x86_64/loongarch64/wasm32 are runtime-detected by default, RISC-V requires additional configuration[^1].

[^1]: https://docs.rs/sha2/latest/sha2/#backends

## Benchmark

* File size: 69.8 GiB
* Storage: Predator GM7000
* OS: Windows 11 x86_64 22620
* CPU: Intel Core i7-12700H
* Hash algorithm: SHA-256

### Output of `openssl speed -evp sha256`

```text
Doing sha256 ops for 3s on 16 size blocks: 15350007 sha256 ops in 2.91s
Doing sha256 ops for 3s on 64 size blocks: 13970470 sha256 ops in 2.97s
Doing sha256 ops for 3s on 256 size blocks: 10049997 sha256 ops in 2.88s
Doing sha256 ops for 3s on 1024 size blocks: 4545898 sha256 ops in 3.00s
Doing sha256 ops for 3s on 8192 size blocks: 743081 sha256 ops in 2.94s
Doing sha256 ops for 3s on 16384 size blocks: 386881 sha256 ops in 2.95s
version: 3.6.0
built on: Wed Oct  8 20:29:58 2025 UTC
options: bn(64,64)
compiler: cl  /Z7 /Fdossl_static.pdb /Gs0 /GF /Gy /MD /W3 /wd4090 /nologo /O2 -DL_ENDIAN -DOPENSSL_PIC -D"OPENSSL_BUILDING_OPENSSL" -D"OPENSSL_SYS_WIN32" -D"WIN32_LEAN_AND_MEAN" -D"UNICODE" -D"_UNICODE" -D"_CRT_SECURE_NO_DEPRECATE" -D"_WINSOCK_DEPRECATED_NO_WARNINGS" -D"NDEBUG" -D_WINSOCK_DEPRECATED_NO_WARNINGS -D_WIN32_WINNT=0x0502
CPUINFO: OPENSSL_ia32cap=0xfffaf38bffcbffff:0x184007a4239c27a9:0x00400810bc18c410:0x0000000000000000:0x0000000000000000
The 'numbers' are in 1000s of bytes per second processed.
type             16 bytes     64 bytes    256 bytes   1024 bytes   8192 bytes  16384 bytes
sha256           84507.57k   301173.92k   894886.69k  1551666.52k  2072279.00k  2146423.98k
```

| Command                          | Time  | Notes               |
| -------------------------------- | ----- | ------------------- |
| `fasthash sha256 -b 1MiB file`   | 36s   | queue=8             |
| `fasthash sha256 -b 256K file`   | 52s   | queue=8             |
| `sha256sum file`                 | 67s   | uutils 0.9.0        |
| hashlib*                         | 68s   | Python 3.13.5       |
| `openssl sha256 file`            | 78s   | OpenSSL 3.6.0 MSVC  |
| --                               | 85s   | Nanazip 6.0.1742    |
| `Get-FileHash file`              | 96.9s | PowerShell 5.1      |
| `sha256sum file`                 | 123s  | Microsoft coreutils |
| `open -r file \| hash sha256sum` | 130s  | NuShell 0.112.2     |


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
