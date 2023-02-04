# SpikyOS

SpikyOS is a hobby operating system written in [Rust](https://www.rust-lang.org/).

## Create a bootable USB (mac)

```shell
cargo build --release

diskutil list

diskutil unmountDisk /dev/disk2

sudo dd if=uefi.img of=/dev/disk2 bs=1m

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



https://github.com/vinaychandra/MoonDustKernel/blob/1918d302c932a9b610944930e38790156d3f7c53/src/arch/x86_64/memory/cpu_local.rs
https://github.com/vinaychandra/MoonDustKernel/blob/1918d302c932a9b610944930e38790156d3f7c53/src/common/memory/cpu_local.rs
