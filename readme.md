
# Simple barebones Rust OS

This example will generate a simple OS kernl bootable by UEFI system

```
rustup install nightly
rustup component add rust-src
rustup target add x86_64-unknown-uefi

cargo +nightly build
cargo build
```

## Reference:
* <https://medium.com/@applepies12/writing-an-os-in-rust-part-1-bb310ff2ee6d>
* <https://medium.com/@applepies12/writing-an-os-in-rust-part-2-8a424f71e6ae>