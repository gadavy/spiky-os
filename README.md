# SpikyOS

SpikyOS is a hobby operating system written in [Rust](https://www.rust-lang.org/).

## Create a bootable USB (mac)

```shell
cargo build --release

diskutil list

diskutil unmountDisk /dev/disk2

sudo dd if=/Users/gadavy/dev/my/spiky-os/target/release/build/spiky-os-962c8ea19b775413/out/uefi.img of=/dev/disk2 bs=1m

diskutil eject /dev/disk2
```

## Links

blog:
https://os.phil-opp.com/testing/

os wiki:
https://wiki.osdev.org/Bare_Bones
https://wiki.osdev.org/PCI

os with syscalls:
https://github.com/Narasimha1997/r3/blob/main/r3_kernel/src/cpu/mod.rs

os from blog, but use bootloader 0.10
https://github.com/HalogenPowered/os/tree/master/src

https://github.com/theseus-os/Theseus/blob/theseus_main/kernel/pci/src/lib.rs

https://nfil.dev/kernel/rust/coding/rust-kernel-to-userspace-and-back/
