# piss-OS

very very bad OS written in rust (ofc), the name is currently a WIP

## In the Future

I plan to develop this to the point of working as a daily driver, so that I can flex that I use my own custom OS.

## How to Build

- To build the kernel, run **`cargo kbuild`**.
- To build the kernel and turn it into a bootable disk image, run **`cargo kimage`** (short for "kernel image"). This will invoke our `boot` sub-crate with an additional `--no-run` argument so that it just creates the disk image and exits.
- To additionally run the kernel in QEMU after creating the disk image, run **`cargo krun`**.
