# Simple barebones Rust OS

This example will generate a simple OS kernl bootable by UEFI system

```
rustup install nightly
rustup component add rust-src
rustup target add x86_64-unknown-uefi

cargo +nightly build
cargo build
```

## В дополнение к авторскому варианту

### компиляция

```
cargo +nightly build --release
```

### подготовить окружение

```
mkdir -p ./boot/EFI/boot
cp ./taret/x86_64-unknown-uefi/release/rustos-boot-uefi.efi ./boot/EFI/boot/Bootx64.efi
```

### запуск с директории ./boot (предварительно скопировать исполняемый файл из target в ./boot/EFI/boot/Bootx64.efi

```
qemu-system-x86_64 -bios /usr/share/ovmf/x64/OVMF.4m.fd -drive format=raw,file=fat:rw:./boot/
```

### загрузка с флешки

```
sudo qemu-system-x86_64 -bios /usr/share/ovmf/x64/OVMF.4m.fd -m 512 -hda /dev/sdX
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
