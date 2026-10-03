# Rust Embedded User-Space Resources

## PYNQ-Z2 official image and later Debian migration

- [PYNQ-Z2 Setup Guide](https://pynq.readthedocs.io/en/latest/getting_started/pynq_z2_setup.html)
  Official SD-boot jumper, power, Micro USB-UART, Ethernet, and first-boot steps.
- [PYNQ supported boards and pre-built images](https://www.pynq.io/boards.html)
  Official board table identifying PYNQ-Z2 and linking its pre-built SD card image.
- [PYNQ SD Card image build guide](https://github.com/Xilinx/PYNQ/blob/master/docs/source/pynq_sd_card.rst)
  Explains how the board-specific boot files, U-Boot, device tree, kernel, and root filesystem fit together.
- [Debian armhf booting guide](https://www.debian.org/releases/stable/armhf/ch05s01.en.html)
  Describes U-Boot, kernel, initrd, and device-tree requirements for 32-bit hard-float ARM systems.
- [Rust Arm Linux support](https://doc.rust-lang.org/rustc/platform-support/arm-linux.html)
  Covers 32-bit Arm ABI selection and cross-linker setup for Linux user-space programs.
- [Community: PYNQ-Z2 U-Boot and Linux kernel](https://gist.github.com/zOrg1331/f1e90e4b31dc668fbe6837a18492a8ee)
  A practical record of board-specific boot artifacts; use it as a supplement to the official files and serial logs.

## Knowledge

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
  Official Rust book. Use for the language foundations, especially ownership, borrowing, structs/enums, and error handling.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  Official runnable examples. Use as a concise companion when looking up syntax introduced in a lesson.
- [Cross-compilation - The rustup book](https://rust-lang.github.io/rustup/cross-compilation.html)
  Explains target standard libraries and why native linker tools are also needed for cross-compilation.
- [`armv7-unknown-linux-gnueabihf` target support](https://doc.rust-lang.org/rustc/platform-support/arm-linux.html)
  Rust's target requirements and support status for 32-bit hard-float Linux on ARM.
- [Cargo configuration reference](https://doc.rust-lang.org/cargo/reference/config.html)
  Use to configure a target-specific linker and other per-target build settings.
- [Debian `gcc-arm-linux-gnueabihf` package](https://packages.debian.org/trixie/gcc-arm-linux-gnueabihf)
  The Debian cross-compiler package for producing ARMv7 hard-float Linux executables from an AMD64 host.
- [Linux SPI userspace API](https://docs.kernel.org/spi/spidev.html)
  Kernel documentation for the `spidev` character-device interface, transfer modes, and configuration.
- [Rust `TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
  Standard-library API for the PYNQ TCP client and its reads, writes, timeouts, and shutdown behavior.
- [PYNQ-Z2 Base Overlay](https://pynq.readthedocs.io/en/v2.7.0/pynq_overlays/pynqz2/pynqz2_base_overlay.html)
  Board-level reference for the PYNQ-Z2 overlay and its peripheral architecture; use when adapting physical tests.

## Wisdom (Communities)

- [Rust Users Forum](https://users.rust-lang.org/)
  Ask focused language and library questions after reducing them to a small reproducible example.
- [PYNQ Support Forum](https://discuss.pynq.io/)
  Ask board, overlay, and Linux peripheral-access questions that depend on a specific PYNQ image or hardware setup.
