# SpikyOS

[![Test Status](https://github.com/gadavy/spiky-os/actions/workflows/test.yaml/badge.svg)](https://github.com/gadavy/spiky-os/actions/workflows/test.yaml)
[![Check Status](https://github.com/gadavy/spiky-os/actions/workflows/check.yaml/badge.svg)](https://github.com/gadavy/spiky-os/actions/workflows/check.yaml)

SpikyOS is a hobby x86_64 operating system written in [Rust](https://www.rust-lang.org/).

This project started from the seventh post of the second edition of [Writing an OS in Rust](https://os.phil-opp.com/) and by
reading the [OSDev wiki](https://wiki.osdev.org/) along with many open source kernels.

## Quick start

On macOS, do the following:

1. Clone repository:
    ```shell
    git clone https://github.com/gadavy/spiky-os.git
    ```

2. Install dependencies:
    ```shell
    brew install nasm
    ```

3. Build and run the kernel in QEMU:
   ```shell
   cargo r -r -- qemu --cpu-count=2
   ```

## Create a bootable USB

Currently only supported on macOS

```shell
sudo cargo r -r -- flash /dev/disk2
```
