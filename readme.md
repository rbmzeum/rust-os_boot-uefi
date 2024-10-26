# Simple barebones Rust OS

This example will generate a simple OS kernl bootable by UEFI system

```
rustup install nightly
rustup component add rust-src
rustup target add x86_64-unknown-uefi

cargo +nightly build
cargo build
```

## testing

- create a directory named `/EFI/boot` and rename boot file as `Bootx64.efi`
- execute Qemmu at root of this directory. Example: `/mnt/w/work/rust-os/rustos-boot-uefi/target/x86_64-unknown-uefi/debug/EFI/boot/Bootx64.efi`

```
qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive format=raw,file=fat:rw:/mnt/w/work/rust-os/rustos-boot-uefi/target/x86_64-unknown-uefi/debug/
```

## Reference:

- <https://medium.com/@applepies12/writing-an-os-in-rust-part-1-bb310ff2ee6d>
- <https://medium.com/@applepies12/writing-an-os-in-rust-part-2-8a424f71e6ae>
