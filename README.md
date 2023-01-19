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

## Create a bootable USB (mac)

```shell
diskutil list

diskutil unmountDisk /dev/disk2

sudo dd if=spiky-os.img of=/dev/disk2 bs=1m

diskutil eject /dev/disk2
```
