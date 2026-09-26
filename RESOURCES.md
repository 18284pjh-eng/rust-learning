# Rust Embedded User-Space Resources

## Knowledge

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
  Official Rust book. Use for the language foundations, especially ownership, borrowing, structs/enums, and error handling.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  Official runnable examples. Use as a concise companion when looking up syntax introduced in a lesson.
- [Cross-compilation - The rustup book](https://rust-lang.github.io/rustup/cross-compilation.html)
  Explains target standard libraries and why native linker tools are also needed for cross-compilation.
- [`aarch64-unknown-linux-gnu` target support](https://doc.rust-lang.org/rustc/platform-support/aarch64-unknown-linux-gnu.html)
  Rust's target requirements and support status for 64-bit Linux on ARM.
- [Cargo configuration reference](https://doc.rust-lang.org/cargo/reference/config.html)
  Use to configure a target-specific linker and other per-target build settings.
- [Debian `gcc-aarch64-linux-gnu` package](https://packages.debian.org/trixie/gcc-aarch64-linux-gnu)
  The Debian cross-compiler package for producing ARM64 Linux executables from an AMD64 host.
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
