# pess-OS
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

hobby OS written in rust (🚀 blazingly fast 🚀).

## Building from Source

### Prequsities
- [QEMU](https://www.qemu.org/)
- Rust (nightly), preferably through [rustup](https://rustup.rs/)

### Step-by-Step
1. First, ensure you have the prerequisites:\
  `qemu-system-x86_64 --version`\
  `rustup show`

2. Second, clone the repository and navigate to it:\
  `git clone https://github.com/hueblu/pess-os ./pess-os && cd ./pess-os`

2. Finally, build and run the kernel:\
  `cargo run`

## TODO

- [x] Setup building with bootloader and running with QEMU
- [ ] Implement VGA buffer to write to the screen
- [ ] Find a way to run unit and integration tests on the kernel
- [ ] Interrupts
  - [ ] Setup an IDT to better handle CPU interrupts
  - [ ] Setup IDT to capture hardware interrupts, to get input from the keyboard and such
  - [ ] Implement IST to handle double faults
- [ ] Memory Management
  - [ ] Paging
  - [ ] Heap Allocating
  

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center"><a href="https://pengalu.me/"><img src="https://avatars.githubusercontent.com/u/72057664?v=4?s=100" width="100px;" alt="Pengalu"/><br /><sub><b>Pengalu</b></sub></a><br /><a href="https://github.com/hueblu/pess-os/commits?author=Pengalu" title="Documentation">📖</a> <a href="#userTesting-Pengalu" title="User Testing">📓</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
